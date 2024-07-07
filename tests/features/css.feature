Feature: A struct exists as an helper for managing CSS classes by using a signal

  Background: Signals are a created from a runtime instance
    Given a created runtime instance

  Rule: An instance of `CssClasses` can be created
    Scenario: An instance of `CssClasses` is created from the runtime instance
      Given the created runtime instance, copy of the reference to the runtime instance
      When a CSS class is created from the runtime instance
      Then the CSS class instance is created

    Scenario: A created instance of `CssClasses` has a empty values
      Given an instance of `CssClasses` created from the runtime instance
      When the value of the CSS classes is get
      Then the value is empty

  Rule: An instance of `CssClasses` can manage CSS classes
    Background: An instance of `CssClasses` will be used for managing CSS classes
      Given an instance of `CssClasses` created from the runtime instance

    Scenario: The value of an instance of `CssClasses` is the list of CSS classes separated by a space
      Given a copy of the instance of `CssClasses`
        And 3 CSS classes is added to the instance of `CssClasses`
      When the value of the CSS class is get
      Then the value is the list of CSS classes separated by a space

    Scenario: A CSS class can be added a CSS class
      Given a copy of the instance of `CssClasses`
      When a CSS class is added to the instance of `CssClasses`
      Then the CSS class is added to the instance of `CssClasses`

    Scenario: A CSS class can be removed a CSS class
      Given a copy of the instance of `CssClasses`
        And a CSS class is added to the instance of `CssClasses`
      When a CSS class is removed from the instance of `CssClasses`
      Then the CSS class is removed from the instance of `CssClasses`

    Scenario: A CSS class can be toggled a CSS class, if the CSS class is not present, it is added
      Given a copy of the instance of `CssClasses`
      When a CSS class is toggled from the instance of `CssClasses`
      Then the CSS class is added to the instance of `CssClasses`

    Scenario: A CSS class can be toggled a CSS class, if the CSS class is present, it is removed
      Given a copy of the instance of `CssClasses`
        And a CSS class is added to the instance of `CssClasses`
      When a CSS class is toggled from the instance of `CssClasses`
      Then the CSS class is removed from the instance of `CssClasses`

    Scenario: A CSS class can be replaced with another CSS class
      Given a copy of the instance of `CssClasses`
        And a CSS class is added to the instance of `CssClasses`
      When the CSS class that was added is replaced with another CSS class
      Then the CSS class is replaced, the old CSS class is removed and the new CSS class is added

    Scenario: CSS classes can be checked if it contains a CSS class, if the CSS class is not present, it goes false
      Given a copy of the instance of `CssClasses`
      When CSS classes is checked if it contains a CSS class
      Then the instance of `CssClasses` does not contain the CSS class

    Scenario: CSS classes can be checked if it contains a CSS class, if the CSS class is present, it goes true
      Given a copy of the instance of `CssClasses`
        And a CSS class is added to the instance of `CssClasses`
      When CSS classes is checked if it contains a CSS class
      Then the instance of `CssClasses` contains the CSS class

  Rule: An instance of `CssClasses` is a signal
    Background: An instance of `CssClasses` will be used for managing CSS classes
      Given an instance of `CssClasses` created from the runtime instance

    Scenario: A change in the CSS classes is notified to an effect
      Given a copy of the instance of `CssClasses`
        And an effect created with this instance of `CssClasses` as a signal
      When a CSS class is added to the instance of `CssClasses`
      Then the effect is notified

    Scenario: An instance of `CssClasses` can create a new instance of `CssClasses` which is linked to the first one
      Given a copy of the instance of `CssClasses`
        And a link is created from this instance of `CssClasses`
      When a new CSS class is added through the link
      Then the instance of `CssClasses` is notified

  Rule: A signal can be attached to an instance of `CssClasses` is a signal to change one CSS class
    Background: An instance of `CssClasses` will be used for managing CSS classes
      Given an instance of `CssClasses` created from the runtime instance

    Scenario: A signal attached to an instance of `CssClasses` notifies changes to the instance of `CssClasses`
      Given a copy of the instance of `CssClasses`
        And a signal attached to this instance of `CssClasses`
      When the signal notifies a change to the CSS classes
      Then the CSS classes have changed

    Scenario: A signal attached to an instance of `CssClasses` replace old value in CSS classes when signal changes
      Given a copy of the instance of `CssClasses`
        And a signal attached to this instance of `CssClasses`
        And a CSS class is set through the signal
      When a new value is set through the signal
      Then the old value is replaced by the new value in the CSS classes
