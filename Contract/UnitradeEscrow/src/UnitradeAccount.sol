// SPDX-License-Identifier: MIT
pragma solidity ^0.8.2 ;

contract UnitradeAccounts{

struct User{
    string userName;
    string phoneNumber;
    address payable userAddress;
}
/**
 * checking if phone is already registered
 */
mapping(string => bool) public isPhoneRegistered;
/**
 * check is address exists
 */
mapping(address => bool) public isAddressRegistered;
/**
 * mapp phone number to address
 */
mapping(string => address) public mappAddresstoPhoneNumber;


mapping(address =>User) public userInfo;
/**
 * check if phone exist
 */
modifier phoneExist(string calldata _phone){
    require(isPhoneRegistered[_phone] == false,"PhoneExist");
    _;
}
/**
 * check if address exist to prevent owning more than one account
 */
modifier addressExist(address _userAddress){
    require(isAddressRegistered[_userAddress] == false,"AddressExist");
    _;
}

    function registerUser(string calldata _username,string calldata _phoneNumber) public addressExist(msg.sender) phoneExist(_phoneNumber){
        userInfo[msg.sender] = User(_username,_phoneNumber,payable(msg.sender));
        mappAddresstoPhoneNumber[_phoneNumber] = msg.sender;
  
    }
 /**
  * getAddress from phone Number of a user
  * 
  */

 function getUserAddress(string calldata _phoneNumber)public view returns(address){
    return  mappAddresstoPhoneNumber[_phoneNumber];
 }
    
}