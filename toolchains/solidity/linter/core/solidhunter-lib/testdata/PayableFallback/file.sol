pragma solidity 0.8.19;

contract Test {
    function() public payable {} // Valid

    fallback() external payable {} // Valid

    function() public {} // Not Valid

    fallback() external {} // Not Valid
}
