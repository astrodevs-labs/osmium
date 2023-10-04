abstract contract One {
    uint storedData;
    function set(uint x) public {
        string myString = "hello";
        storedData = x;
    }

    function get() public view returns (uint) {
        return storedData;
    }
    uint8 constant myConst = 1;
}

abstract contract Two {
    function set(uint x) public {
        uint storedData = One.myConst;
        storedData = x;
    }

    function get() public view returns (uint) {
        return storedData;
    }
}