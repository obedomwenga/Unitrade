// SPDX-License-Identifier: MIT
pragma solidity ^0.8.2;
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
contract UnitradeEscrow {
    constructor(){
        isAdmin[msg.sender]=true;
    }
    struct Trade{
        address payable  sender;
        address payable  reciever;
        uint   amount;
        bool disputed;

    }
address[] public admins;
//mapping of  a trade
mapping(uint => Trade)public trade;
//is Admin
mapping(address => bool)public isAdmin;
//track balance
mapping(address => uint)public clientBalance;
//mapping trade number  and dispute
mapping(uint=>mapping(address => Trade)) public tradeInDispute;
//tradeNumber
uint tradeNumber;
//only admins
// Modifier to check if the sender is an admin
    modifier onlyAdmins() {
       
       
        require(isAdmin[msg.sender], "Only admins can perform this action.");
        _;
    }
function setAdmins(address _admin)public onlyAdmins {
    isAdmin[_admin]= true;

}
function revokeAdmins(address _admin)public onlyAdmins {
    isAdmin[_admin]= false;

}

function unitradeescrow(address _ierc20Token,uint  _amount,address payable _receiver)external{
    uint _tradeIndex = tradeNumber;
 IERC20(_ierc20Token).transferFrom(msg.sender,address(this),_amount);

 clientBalance[msg.sender] = clientBalance[msg.sender] + _amount;
 tradeInDispute[_tradeIndex][msg.sender] = Trade(payable(msg.sender),_receiver,_amount,false);
}

function raisedispute(uint _tradeNumber)public{
    tradeInDispute[_tradeNumber][msg.sender].disputed = true;


}
function releaseToVendor(address _ierc20Token,uint _amount,address payable _receiver,uint _tradeNumber)public{
 require( tradeInDispute[_tradeNumber][msg.sender].disputed ==false ,"trade has an issue");
 payout(_ierc20Token, _amount, _receiver);
}
function settleDispute(uint _tradeNumber, address payable _issuer,address _ierc20Token)public onlyAdmins{
    tradeInDispute[_tradeNumber][_issuer].disputed = false;
   refund(_ierc20Token, tradeInDispute[_tradeNumber][_issuer].amount,_issuer);

}
function refund(address _ierc20Token,uint  _amount, address payable _issuer)internal{
    IERC20(_ierc20Token).transfer(_issuer,_amount);
}
    function payout(address _ierc20Token,uint _amount,address payable _receiver)internal{
        IERC20(_ierc20Token).transferFrom(msg.sender,_receiver,_amount);
    }

}