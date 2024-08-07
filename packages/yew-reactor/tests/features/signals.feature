Feature: Signals for Yew

  Having signals for Yew, and having reactivity in my web app developed with Yew.

  Background: Signals are a created from a runtime instance
    Given a created runtime instance

  Rule: A signal can be accessed from a copy, this is the right way to access the signal
    Background: A signal is created from the runtime instance
      Given a signal created from the runtime instance

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
      Then the transformed value should be returned

    Scenario: A signal value can be set through a function which modifying the underlaying signal value
      Given a copy of the signal
      When the value is set to the signal through a function
      Then the value of the signal should be set to the transformed value

    Scenario: Two signals can be combined through a function
      Given a copy of the signal
        And another signal is created from the runtime instance
      When the two signals are combined through a function
      Then the combined value should be returned

  Rule: A signal can be subscribed to an effect
    Background: A signal is created from the runtime instance
      Given a signal created from the runtime instance

    Scenario: A signal is subscribed to an effect and should notify the effect at its creation
      Given a copy of the signal
      When the signal is subscribed to a new effect created with the runtime instance
      Then the effect is called

    Scenario: A signal is subscribed to an effect and should not notify when the signal has an untracked change
      Given the signal is subscribed to a new effect created with the runtime instance
      When the signal changes its value with an untracked change
      Then the modification should not notify the effect

    Scenario: A signal is subscribed to an effect and should notify the effect when another value is set
      Given the signal is subscribed to a new effect created with the runtime instance
      When the signal changes its value
      Then the modification should notify the effect

    Scenario: A signal is subscribed to an effect and should notify the effect when the set value does not change
      Given the signal is subscribed to a new effect created with the runtime instance
      When the signal set a value that is the same as the previous value
      Then the modification should notify the effect

    Scenario: A signal is subscribed to an effect and should notify the effect with a conditioned change
      Given the signal is subscribed to a new effect created with the runtime instance
      When the signal changes its value through a conditioned change that requests an update
      Then the modification should notify the effect

    Scenario: A signal is subscribed to an effect and should notify the effect when the conditioned change is not met
      Given the signal is subscribed to a new effect created with the runtime instance
      When the signal changes its value through a conditioned change that doesn't request an update
      Then the modification should not notify the effect

  Rule: A signal can be linked to another signal and the linked signal acts as the source signal
    Background: A signal is created from the runtime instance
      Given a signal created from the runtime instance

    Scenario: The linked signal should be updated when the source signal changes
      Given a copy of the signal, the source signal [S]
        And a new signal is created from the runtime instance, the destination signal [D]
      When the new signal D is linked to the signal S
        And a value is set to the signal S
      Then the signal D should be updated when the value of the signal S changes

    Scenario: Update the linked signal should update the source signal
      Given a copy of the signal, the source signal [S]
        And a new signal is created from the runtime instance, the destination signal [D]
      When the new signal D is linked to the signal S
        And a value is set to the signal D
      Then the signal S should be updated when the value of the signal D changes

    Scenario: When the linked signal is updated, effects subscribed to the source signal should be called
      Given a copy of the signal, the source signal [S]
        And a new signal is created from the runtime instance, the destination signal [D]
        And the new signal D is linked to the signal S
        And an effect is created from the signal S
      When a value is set to the signal D
      Then the effect should be called

    Scenario: Several signals can be linked to a single source signal, and all signals will be updated
      Given a copy of the signal, the source signal [S]
        And another signal is created from the runtime instance, the first destination signal [D1]
        And another signal is created from the runtime instance, the second destination signal [D2]
        And the first destination signal D1 is linked to the source signal S
      When the second destination signal D2 is linked to the source signal S
        And a value is set to the source signal
      Then the first destination signal D1 should be updated
        And the second destination signal D2 should be updated

    Scenario: A linked signal can be linked to another signal which is a link destination
      Given a copy of the signal, the source signal [S]
        And a new signal is created from the runtime instance, the destination signal [D]
        And the new signal D is linked to the signal S
      When a link is created from the signal D, the new destination signal [H]
        And a value is set to this link, the signal H
      Then the signal S should be updated
        And the signal D should be updated
