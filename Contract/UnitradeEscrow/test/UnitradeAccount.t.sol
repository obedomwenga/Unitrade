// SPDX-License-Identifier: MIT
pragma solidity ^0.8.2 ;

import {UnitradeAccounts} from "../src/UnitradeAccount.sol";
import "forge-std/Test.sol";

contract UnitradeAccountsTest is Test{
    UnitradeAccounts unitrade;
    function setUp() public{
        unitrade = new UnitradeAccounts();

    }
    function test_RegisterUser() public{
        string memory number ="071234567";
        string memory username ="yollow";
        vm.prank(address(1));
        unitrade.registerUser(username,number);
       assertEq(unitrade.getUserAddress(number),address(1));
    }
}