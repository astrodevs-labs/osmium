pragma solidity 0.8.0;

contract Test {
    function combineToFunctionPointer(address newAddress, uint256 newSelector)
        public
        pure
        returns (function() external fun)
    {
        assembly {
            fun.selector := newSelector
            fun.address := newAddress
        }
    }
}
