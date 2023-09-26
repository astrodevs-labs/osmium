File :
    * Imports (name):
    * Contract { definition } (name, Inheritance): contract Test is ERC20 {}
        - ContractName(contract)
        - ContractInheritance(contract, inheritance)

    * Contract { instantiation } (type) : new Test(param1, param2)
        - ContractInstantiation(file, contract, Option<function>, new)

    * Contract { scope } (type) : Test::Enum1::ONE
        - ContractScope(file, contract, Option<function>, scope) //TODO: search contract scope as
    - Contract: 
        * Function { definition } (name, return_type, parameters)
            - ReturnType(file, contract, function, type)
            - FunctionParameter(file, contract, function, parameter)
            - FunctionName(file, contract, function)
        * Function { usages } (name)
            - FunctionName(file, contract, function)

        * Property { definition } (name, type)
            - PropertyName(file, contract, property, name)
            - PropertyType(file, contract, property, type)

    * Variable { definition } (name, type)
        - VariableName(file, Option<contract>, Option<function>, variable)
        - VariableType(file, Option<contract>, Option<function>, variable)
            
    * Variable { usages } (name)
        - VariableName(file, contract, function, property)
    
    Enum
    Struct
    Event
        Emit
    Error(e en miniscule)
    Library
        usage
    // TODO: search cast ast node
    // TODO: search super ast node




contract Test {
modifier onlyOwner() {
...
}

function a(param1) public return uint256 onlyOwner {
..
}