pragma solidity 0.8.0;

contract Test {
    function awesome() public {
        require(!has(role, account), "This is perfect");
    }
    function not_awesome() public {
        require(!has(role, account), "This is not perfect at all because it's really too long but the code is 0xSwapFeeder compliant");
    }
    function not_awesome_either() public {
        require(!has(role, account));
    }
}