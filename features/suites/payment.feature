Feature: payment

  This repository's Prism application, in app/: given a role, an amount and a
  limit, it answers whether the payment is approved or refused.

  @PA-01 @build
  Scenario: The app never approves a clerk's payment whose amount exceeds the limit
    Given the payment approval model in app/src/PaymentApproval.lex.tex
    When the Lean kernel checks inv001Holds during prismpm verify
    Then no request from a clerk with an amount above the limit is judged approved

  @PA-02 @build
  Scenario: The app always approves a manager's payment
    Given the payment approval model in app/src/PaymentApproval.lex.tex
    When the Lean kernel checks inv002Holds during prismpm verify
    Then every request from a manager is judged approved
