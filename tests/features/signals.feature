Feature: Signals for Yew

  Having signals for Yew, and having reactivity in my web app developed with Yew.

  Background: Signals are a created from a runtime instance
    Given a created runtime instance

  Rule: A signal can be accessed from a copy, this is the right way to access the signal
    Background: A signal copy will be used to access the signal
      Given a signal is created from the runtime instance

    Scenario: A signal value can be got
      Given the copy of the signal
      When a value is got from the signal
      Then the value of the signal should be returned

    Scenario: A signal value can be set
      Given a copy of the signal
      When a value is set to the signal
      Then the value of the signal should be set

    Scenario: A signal value can be got through a function which maps the underlaying signal value
      Given a copy of the signal
      When the value is got from the signal and transformed through a function
      Then the transfomed value should be returned

    Scenario: A signal value can be set through a function which modifying the underlaying signal value
      Given a copy of the signal
      When the value is set to the signal through a function
      Then the value of the signal should be set

    Scenario: Two signals can be combined through a function
      Given a copy of the signal
        And another signal is created from the runtime instance
      When the two signals are combined through a function
      Then the combined value should be returned

  Rule: A signal can be subscribed to an effect
    Background: A signal copy will be used to access the signal
      Given a signal is created from the runtime instance

    Scenario: A signal is subscribed to an effect and should notify the effect when the signal value is set
      Given the signal is subscribed to a new effect created with the runtime instance
      When the signal set its value
      Then the modification should notify the effect

    Scenario: A signal is subscribed to an effect and should not notify when the signal has an untracked change
      Given the signal is subscribed to a new effect created with the runtime instance
      When the signal changes its value with an untracked change
      Then the modification should not notify the effect

    Scenario: A signal is subscribed to an effect and should notify the effect when the signal value changes
      Given the signal is subscribed to a new effect created with the runtime instance
      When the signal changes its value
      Then the modification should notify the effect

    Scenario: A signal is subscribed to an effect and should notify the effect when the signal value does not change
      Given the signal is subscribed to a new effect created with the runtime instance
      When the signal set a value that is the same as the previous value
      Then the modification should notify the effect

  Rule: A signal can be linked to another signal
    Background: A signal copy will be used to access the signal
      Given a signal is created from the runtime instance

    Scenario: A signal can be linked to another signal
      Given a copy of the signal, the signal source (S)
        And a new signal is created from the runtime instance, the signal destination (D)
      When the signal S is linked to the new signal D
        And a value is set to the signal S
      Then the signal D should be updated when the value of the signal S changes

    Scenario: A linked signal can be linked to another signal which is not a link source
      Given a copy of the signal, the first signal source (S)
        And another signal is created from the runtime instance, the first signal destination (D1)
        And another signal is created from the runtime instance, the second signal destination (D2)
        And the signal S is linked to the first signal destination D1
      When the signal source S is linked to the second signal destination D2
        And a value is set to the signal source
      Then the first signal destination D1 should not be updated
        And the second signal destination D2 should be updated

    Scenario: A linked signal can be linked to another signal which is a link source
      Given a copy of the signal, the signal destination (D)
        And a link is created from the signal D, the signal source (S)
      When a link is created from the signal S
        And a value is set to this link
      Then the signal destination should be updated
