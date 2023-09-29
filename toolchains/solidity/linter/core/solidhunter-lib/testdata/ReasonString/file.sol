pragma solidity 0.8.0;

contract Test {
    function awesome() public {
        require(!has(role, account), "This is perfect");
    }
    function notAwesome() public {
        require(!has(role, account), "This is not perfect at all because i");
    }
    function notAwesomeEither() public {
        require(!has(role, account));
    }
}