# grove-skill-delivery-k16

**Reviews:** grove-skill-delivery-k8

## Goal

Adversarially check Grove's corrected skill-delivery design before implementation
planning treats it as a settled contract.

## Context

Read the producer's commit by its stable handle and the current requirements.
The human corrected the design. Grove names a skill. The user supplies it
natively or through Ravel directory injection, and the agent finds and reads it.
Ravel does not resolve a requested skill. Consume ravel-skill-directories-k15
and any integrated review of it, including its owning-repository commits.

The artifact set is Grove's delivery and caller specs, related ADRs, glossary
notes, design evidence and caller diagrams. Reconcile the producer's historical
diff with any deliberate upstream correction. Do not review the discarded
catalog/Ravel-only proposal as if it were still the artifact.

## Done when

- Check that native installation and injected directories are user choices that
  the design supports. Check that no skill presence, location, catalog or kind
  registry is in Grove's public contract.
- Challenge fresh bootstrap, missing-skill reporting, relative and cross-pack
  resources, conformance boundaries and native-hook coexistence against the
  corrected Ravel contract. A readable directory must not stand in for actual
  agent discovery or usable native integrations.
- Challenge old-reader preservation, explicit cleanup ownership, dual-major
  session compatibility, versioned policy examples and publication/activation
  order. Check that original live ending commands remain usable.
- Assess whether the source-consumer/documentation map and acceptance seams let
  common-skill-loading-k9 cut working increments without inventing behaviour.
  Keep implementation, harness measurements and documentation validation distinct.
- Evaluate the ADR set for necessary, nonduplicated decisions. Produce findings
  only, classifying genuine contradictions separately from visible trade-offs.

## Notes

Read-only review. Cut an integrate-review-design leaf only for findings worth
acting on, immediately before the first later live sibling. This review spends
no in-session reviewer. Implementation remains the existing planner's work.
