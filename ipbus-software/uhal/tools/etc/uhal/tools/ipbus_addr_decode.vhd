---------------------------------------------------------------------------------
-- Address decode logic for IPbus fabric.
--
-- This file has been AUTOGENERATED from the address table - do not
-- hand edit.
--
-- We assume the synthesis tool is clever enough to recognise
-- exclusive conditions in the if statement.
---------------------------------------------------------------------------------

library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

package ipbus_decode_PACKAGENAME is

-- INSERT_SEL_WIDTH_HERE
  subtype ipbus_sel_t is std_logic_vector(IPBUS_SEL_WIDTH - 1 downto 0);
  function ipbus_sel_PACKAGENAME(addr : in std_logic_vector(31 downto 0)) return ipbus_sel_t;

-- INSERT_SYMBOLIC_CONSTANTS_HERE

end ipbus_decode_PACKAGENAME;

package body ipbus_decode_PACKAGENAME is

  function ipbus_sel_PACKAGENAME(addr : in std_logic_vector(31 downto 0)) return ipbus_sel_t is
    variable sel: ipbus_sel_t;
  begin

-- INSERT_SELECTION_HERE
    return sel;

  end function ipbus_sel_PACKAGENAME;

end ipbus_decode_PACKAGENAME;

---------------------------------------------------------------------------------
