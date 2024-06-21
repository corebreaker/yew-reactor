Feature: Actions for using asynchronous functions

  Actions are objects that represent the intention to perform an operation.
  They are created with an async function and can be dispatched to be executed.
  A state (named `pending`) can be used to track the status of the action.

  An action has an input type and an output type.
  The input type is given as argument to the async function.
  The output type is the return type of the async function.

  The dispatching of the action will trigger the execution of the async function with its input type as argument.

  After the execution of the async function, the action will updated the value with its return value.
  This return value will be stored in the action as a signal and can be accessed by the user.

  At the creation of the action, the signal value will be set to `None`,
  so the signal has the type `Option<O>`, with O, the output type.

  Background: Signals are a created from a runtime instance
    Given a created runtime instance

  Rule: An action must be created from an asynchronous function
    Scenario: Create an action from an async function
      Given an async function
      When an action is created from the async function
      Then the action is created with a pending state set as false
        And the value stored in the action is None

  Rule: The async function is called by dispatching its action
    Scenario: Dispatch an action
      Given an async function that returns a value
        And an action created from the async function
      When the action is dispatched
      Then the pending state is set to true before the execution of the async function
        And the stored value is None

    Scenario: An dispatched action can be dispatched again but the function is not executed again
      Given an async function that returns a value
        And an action created from the async function
        And the action is dispatched
      When the action is dispatched again
      Then the pending state stay to true
        And the stored value is None

    Scenario: After the execution of the async function, the value is stored in the action
      Given an async function that returns a value
        And an action created from the async function
        And the action is dispatched
      When the async function is executed
      Then the stored value is the return value of the async function

  Rule: Actions can be used to trigger effects with the return value of the async function
    Scenario: Terminated actions notify effect subscribers with the return value of the async function
      Given an async function that returns a value
        And an action created from the async function
        And an effect is created from the signal stored in the action
      When the action is dispatched
        And the async function has been executed
      Then the return value of the async function is stored in the action
        And the effect is notified with the return value of the async function
