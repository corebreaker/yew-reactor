Feature: Memo function for Yew

  Creating a signal signal from a memo function.
  The signal wraps a value that is calculated from a function.
  Then, when the function returns a new value which has changed,
  the signal is updated and notifies its subscribers.

  Background: Signals are a created from a runtime instance
    Given a created runtime instance

  Rule: Creating a memo function will create a signal which will notify subscribers when the function returns changes
    Background: A signal copy will be used to access the signal
      Given a signal is created from the runtime instance

    Scenario: Creating a memo function will call the function to get the initial value
      Given a function that returns a value
      When the memo function is created
      Then the function is called to get the initial value

    Scenario: The memo function which consume a signal will notify subscribers when the signal value changes
      Given the signal created from a memo function with an initialized value which is computed from the signal
      When the signal value has changed
      Then the function is called to get the value from the signal
        And the signal notifies its subscribers with the new value

    Scenario: A memo function which consume a signal won't notify subscribers when the signal sets an unchanged value
      Given the signal created from a memo function with an initialized value which is computed from the signal
      When the same value is set to the signal
      Then the function is called to get the value from the signal
        And the signal does not notify its subscribers

  Rule: A keyed signal is a memo function which will notify subscribers when the collection changes its value for a key
    Background: A keyed signal will be created from a collection signal, the values will not be stringified
      Given the value for the key in the collection used will not be stringified

    Scenario: Creating a keyed signal will call the function to get the initial value
      Given a signal is created from the runtime instance with a collection signal and a defined key
      When the keyed signal is created
      Then the initial value for the key in the collection si set to the collection signal

    Scenario: A keyed signal won't notify subscribers when the collection signal sets a value that is unchanged
      Given a signal is created from the runtime instance with a collection signal and a defined key
        And an effect created with the keyed signal
      When the same value located at the key in the collection is set through the collection signal
      Then the keyed signal does not notify its subscribers

    Scenario: The keyed signal will notify subscribers when the collection signal value changes for the key
      Given a signal is created from the runtime instance with a collection signal and a defined key
        And an effect created with the keyed signal
      When the signal value located at the key in the collection has changed
      Then the keyed signal notifies its subscribers with the new value

  Rule: A stringified keyed signal is a memo function which will notify subscribers when the function returns changes
    Background: A keyed signal will be created from a collection signal, the values will be stringified
      Given the value for the key in the collection used will be stringified

    Scenario: Creating a keyed signal will call the function to get the initial value
      Given a signal is created from the runtime instance with a collection signal and a defined key
      When the keyed signal is created
      Then the initial value for the key in the collection si set to the collection signal

    Scenario: A keyed signal won't notify subscribers when the collection signal sets a value that is unchanged
      Given a signal is created from the runtime instance with a collection signal and a defined key
        And an effect created with the keyed signal
      When the same value located at the key in the collection is set through the collection signal
      Then the keyed signal does not notify its subscribers

    Scenario: The keyed signal will notify subscribers when the collection signal value changes for the key
      Given a signal is created from the runtime instance with a collection signal and a defined key
        And an effect created with the keyed signal
      When the signal value located at the key in the collection has changed
      Then the keyed signal notifies its subscribers with the new value
