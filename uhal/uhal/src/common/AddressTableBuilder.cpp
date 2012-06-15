#include "uhal/ClientImplementation.hpp"

#include "uhal/AddressTableBuilder.hpp"
#include "uhal/Utilities.hpp"
#include "log/log.hpp"

#include <boost/algorithm/string.hpp>

namespace uhal
{

	AddressTableBuilder* AddressTableBuilder::mInstance = NULL;

	AddressTableBuilder& AddressTableBuilder::getInstance()
	{
		try
		{
			if ( mInstance == NULL )
			{
				mInstance = new AddressTableBuilder();
			}

			return *mInstance;
		}
		catch ( const std::exception& aExc )
		{
			log ( Error() , "Exception \"" , aExc.what() , "\" caught at " , ThisLocation() );
			throw uhal::exception ( aExc );
		}
	}

	boost::shared_ptr< const Node > AddressTableBuilder::getAddressTable ( const std::string& aFilenameExpr , const uint32_t& aAddr , const uint32_t& aAddrMask )
	{
		try
		{
			std::vector< std::pair<std::string, std::string> >  lAddressFiles;
			uhal::utilities::ParseSemicolonDelimitedUriList<true> ( aFilenameExpr , lAddressFiles );

			if ( lAddressFiles.size() != 1 )
			{
				log ( Error() , "Exactly one address table file must be specified. The expression \"" , aFilenameExpr , "\" contains " , Integer ( lAddressFiles.size() ) , " valid file expressions." );
				log ( Error() , "Throwing at " , ThisLocation() );
				throw IncorrectAddressTableFileCount();
			}

			std::vector< boost::shared_ptr< const Node > > lNodes;

			if ( !uhal::utilities::OpenFile ( lAddressFiles[0].first , lAddressFiles[0].second , boost::bind ( &AddressTableBuilder::CallBack, boost::ref ( *this ) , _1 , _2 , _3 , aAddr , aAddrMask , boost::ref ( lNodes ) ) ) )
			{
				log ( Error() , "Failed to open address table file \"" , lAddressFiles[0].second , "\"" );
				log ( Error() , "Throwing at " , ThisLocation() );
				throw FailedToOpenAddressTableFile();
			}

			if ( lNodes.size() != 1 )
			{
				log ( Error() , "Exactly one address table file must be specified. The expression \"" , lAddressFiles[0].second , "\" refers to " , Integer ( lNodes.size() ) , " valid files." );
				log ( Error() , "Throwing at " , ThisLocation() );
				throw IncorrectAddressTableFileCount();
			}

			return lNodes[0];
		}
		catch ( const std::exception& aExc )
		{
			log ( Error() , "Exception \"" , aExc.what() , "\" caught at " , ThisLocation() );
			throw uhal::exception ( aExc );
		}
	}


	void AddressTableBuilder::CallBack ( const std::string& aProtocol , const boost::filesystem::path& aPath , std::vector<uint8_t>& aFile , const uint32_t& aAddr , const uint32_t& aAddrMask , std::vector< boost::shared_ptr< const Node > >& aNodes )
	{
		try
		{
			std::string lName( aProtocol+ ( aPath.string() ) );
			std::hash_map< std::string , boost::shared_ptr< const Node > >::iterator lNodeIt = mNodes.find ( lName );

			if ( lNodeIt != mNodes.end() )
			{
				aNodes.push_back ( lNodeIt->second );
				return;
			}

			std::string lExtension ( aPath.extension().string().substr ( 0,4 ) );
			boost::to_lower ( lExtension ); //just in case someone decides to use capitals in their file extensions.

			if ( lExtension == ".xml" )
			{
				log ( Info() , "XML file" );
				pugi::xml_document lXmlDocument;
				pugi::xml_parse_result lLoadResult = lXmlDocument.load_buffer_inplace ( & ( aFile[0] ) , aFile.size() );

				if ( !lLoadResult )
				{
					uhal::utilities::PugiXMLParseResultPrettifier ( lLoadResult , aPath , aFile );
					return;
				}

				pugi::xml_node lXmlNode = lXmlDocument.child ( "node" );

				if ( !lXmlNode )
				{
					log ( Error() , "No XML node called \"node\" in file " , aPath.c_str() );
					return;
				}

				boost::shared_ptr< const Node > lNode( new Node( lXmlNode , aAddr , aAddrMask ) );
				mNodes.insert( std::make_pair( lName , lNode ) );
				aNodes.push_back ( lNode );
				return;
			}
			else if ( lExtension == ".txt" )
			{
				log ( Info() , "TXT file" );
				log ( Error() , "Parser problems mean that this method has been disabled. Please fix me! Please?!?" );
				log ( Error() , "At " , ThisLocation() );
				return;
				/*
				uhal::OldHalEntryGrammar lGrammar;
				uhal::OldHalSkipParser lParser;
				std::vector< utilities::OldHalEntryType > lResponse;

				std::vector<uint8_t>::iterator lBegin( aFile.begin() );
				std::vector<uint8_t>::iterator lEnd( aFile.end() );

				boost::spirit::qi::phrase_parse( lBegin , lEnd , lGrammar , lParser , lResponse );

				for( std::vector< utilities::OldHalEntryType >::iterator lIt = lResponse.begin() ; lIt != lResponse.end() ; ++lIt ){
					//log ( Info() , "---------------------------------------------------\n" , *lIt );
				}

				//log ( Info() , "Remaining:" );
				for( ; lBegin != lEnd ; ++lBegin ){
					//log ( Info() , *lBegin;
				}
				std::cout );
				*/
			}
			else
			{
				log ( Error() , "Extension \"" , lExtension , "\" not known." );
				return;
			}
		}
		catch ( const std::exception& aExc )
		{
			log ( Error() , "Exception \"" , aExc.what() , "\" caught at " , ThisLocation() );
			throw uhal::exception ( aExc );
		}
	}



}
