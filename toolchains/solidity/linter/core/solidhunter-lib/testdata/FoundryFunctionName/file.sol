pragma solidity 0.8.0;

contract Test {
    function test_NumberIs42() public {} // should pass

    function testFail_Subtract43() public {} // should pass

    function testFuzz_FuzzyTest() public {} // should pass

    function numberIs42() public {} // should fail
}
