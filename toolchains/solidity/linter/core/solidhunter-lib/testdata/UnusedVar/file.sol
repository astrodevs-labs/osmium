pragma solidity 0.8.19;

contract Test {
    uint used = 1;
    uint c = 1;

    function test_fn() public pure {
        string memory str = "test";
    }

    function test_fn2() public view {
        uint b = used;
    }
}
