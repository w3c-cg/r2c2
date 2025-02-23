# DRAFT RDF Rust Common Crates Community Group Charter

[*{TBD: remove next sentence before submitting for approval}*]
This Charter is work in progress. To submit feedback, please
submit an issue to https://github.com/w3c-cg/r2c2/ .

-   This Charter: https://github.com/w3c-cg/r2c2/blob/main/Charter.md
-   Previous Charter: N/A
-   Start Date: 2024-02-01 [*{TBD estimation}*]
-   Last Modified: (see [github history](https://github.com/w3c-cg/r2c2/commits/main/Charter.md))

## Goals

The mission of the RDF Rust Common Crates (<abbr>R2C2</abbr>)
Community Group is to develop a **common API** for working with [RDF] in
[Rust], published as a set of [library crates].
The goal is to improve the interoperability of the RDF ecosystem in Rust.

## Scope of Work

The common API developed by the Community Group will be defined as a set of traits and/or lightweight type,
that other crates are encouraged to implement and reuse,
so that data values produced by an implementation can be reused by another one.

For example, the triples produced by a [Turtle]
parser, provided that they implement a trait defined by this Community Group,
could be ingested by any independently developed implementation of a graph store.
Similarly, if the latter implements the appropriate traits,
it could be checked by any independently developed [SHACL] validator.

As much as possible,
the crates developed by the Community Group should aim at fulfilling the
*[zero-cost abstraction]* motto:
the provided traits should ideally be generic enough,
to be directly implementable by any specific implementation,
regardless of its internal design
(as opposed to requiring a [wrapper type]).
However, overly generic code can be hard too use in practice,
so the Community Group will have to strike a balance between genericity and usability.

The Community Group may also develop utility [library crates], i.e.,
crates providing common types or functions expected to be pervasively used in the implementations of the common API.
For example, the Community Group may produce a crate for parsing and resolving IRIs.

### Out of Scope

* develop an complete implementation of RDF-related standards

## Deliverables

### Rust Crates

The main deliverables of the community group are Rust [library crates],
which should eventually be published on the standard repository https://crates.io/.

### Specifications

No Specifications will be produced under the current charter.

### Non-Normative Reports

The community group MAY produce a user manual, or other kind of documentation,
for the produced crates.

<!-- covered by Rust Crates above
### Test Suites and Other Software

*{TBD: If there are no plans to create a test suite or other software,
please state that and remove the following paragraph. If Github is not
being used, then indicate where the license information is. If GitHub is
being used link to your LICENSE.md file in the next paragraph.}*

The group MAY produce test suites to support the Specifications. Please
see the GitHub LICENSE file for test suite contribution licensing
information.
-->

## Dependencies or Liaisons

* the [RDF Javascript Libraries Community Group] has a similar goal to this Community Group,
  applied to the Javascript programming language.
  Interaction between the two groups could be mutually valuable,
  to share experience and lessons learned.
* [all projects](https://github.com/search?q=rdf+language%3ARust+pushed%3A%3E2024-01-01+&type=repositories)
  implementing RDF (or related standards) is a potential candidate for implementing the R2C2 traits.
  The Community Group will strive to include the maintainers of these project in the discussions.

## Community and Business Group Process

The group operates under the [Community and Business Group
Process](https://www.w3.org/community/about/process/). Terms in this
Charter that conflict with those of the Community and Business Group
Process are void.

As with other Community Groups, W3C seeks organizational licensing
commitments under the [W3C Community Contributor License Agreement
(CLA)](https://www.w3.org/community/about/process/cla/). When people
request to participate without representing their organization\'s legal
interests, W3C will in general approve those requests for this group
with the following understanding: W3C will seek and expect an
organizational commitment under the CLA starting with the individual\'s
first request to make a contribution to a group
[Deliverable](#deliverables). The section on [Contribution
Mechanics](#contribution-mechanics) describes how W3C expects to monitor these
contribution requests.

The [W3C Code of Ethics and Professional
Conduct](https://www.w3.org/Consortium/cepc/) applies to participation
in this group.

<!-- NOT RELEVANT AS WE HAVE NO DELIVERABLE -- insert again if the charter is modified to add specifications
## Work Limited to Charter Scope

The group will not publish Specifications on topics other than those
listed under [Specifications](#specifications) above. See below for [how
to modify the charter](#amendments-to-this-charter).
-->

## Contribution Mechanics

Substantive Contributions to Specifications can only be made by
Community Group Participants who have agreed to the [W3C Community
Contributor License Agreement
(CLA)](https://www.w3.org/community/about/process/cla/).

Deliverables created in the Community Group must use the [W3C Software
and Document
License](http://www.w3.org/Consortium/Legal/2015/copyright-software-and-document).

Community Group participants agree to make all contributions in the
GitHub repository the group is using for the particular source code or document. This may be
in the form of a pull request (preferred), by raising an issue, or by
adding a comment to an existing issue.

All GitHub repositories attached to the Community Group must contain a
copy of the
[CONTRIBUTING](https://github.com/w3c/licenses/blob/master/CG-CONTRIBUTING.md)
and [LICENSE](https://github.com/w3c/licenses/blob/master/CG-LICENSE.md)
files.

## Transparency

The group will conduct all of its technical work in public. If the group
uses GitHub, all technical work will occur in its GitHub repositories
(and not in mailing list discussions). This is to ensure contributions
can be tracked through a software tool.

Meetings may be restricted to Community Group participants, but a public
summary or minutes must be posted to the group\'s public mailing list,
or to a GitHub issue if the group uses GitHub.

## Decision Process

This group will seek to make decisions where there is consensus, as described in this section.

### Work item owner

* Every work item (crate or report) worked on by the Community Group will have a list of owners which will always include the Chairs.
* This list of owners will be recorded using GitHub's [CODEOWNERS] mechanism.
* Only the Chairs are the owners of the CODEOWNERS file.
* Any Community Group participant can ask to be promoted owner of a work item once a pull-request they have submitted on this work item is merged
  (this rule is loosely inspired by the [pull-request hack]).
* The Chairs may demote an owner of a work item who has not contributed (with pull-requests or reviews) to that item in the past year.

### New work item

* Any Community Group participant may propose a new work item (crate or report) by making a pull-request on the Community Group's GitHub repository
  with the label `new-work-item`, which constitutes a call for consensus.
* No less than two weeks after the pull-request was posted,
  the Chairs assess consensus based on the feedback on the pull-request,
  and record a group decision on GitHub, following [Section 5.2] of the W3C process.
* If the decision is to accept the new work item, the Chairs merge the pull-request and promote its author owner of the new work item.
* Regardless of the consensus of its participant, the Community Group can not accept a work item that is not in its [scope](#scope-of-work),
  as defined by this charter.

### Merging pull-requests

* Every Community Group participant will have [write access] to the group's GitHub repository,
  but merging a pull-request will require approval by at least two owners of the affected work item.
* To ensure the applicability of the rule above,
  pull-requests spanning multiple work items are strongly discouraged,
  but the Chairs may decide to allow them on a per-case basis.
* Once sufficiently approved by the appropriate owners (as defined above),
  a pull-request can be merged even if there is disagreement about it.
  Individuals who disagree with the choice are strongly encouraged to take ownership of their objection by submitting a subsequent pull-request.

### Publishing work

* The Chairs are collectively the owners on [crates.io] of the crates released by the Community Group.
* To request the release of a crate on crates.io, or the publication of a report as a [Final Community Group Report],
  a Community Group participant opens a pull-requests or an issue with the label `request-publication`,
  which constitutes a call for consensus.
* No less than two weeks after the pull-request / issue was posted,
  the Chairs assess consensus based on the feedback on the pull-request,
  and record a group decision on GitHub, following [Section 5.2] of the W3C process,
  and take the appropriate action to implement the decision.

It is the Chairs\' responsibility to ensure that the decision process is
fair, respects the consensus of the CG, and does not unreasonably favour
or discriminate against any group participant or their employer.

## Chair Selection

Participants in this group choose their Chair(s) and can replace their
Chair(s) at any time using whatever means they prefer. However, if 5
participants, no two from the same organisation, call for an election,
the group must use the following process to replace any current Chair(s)
with a new Chair, consulting the Community Development Lead on election
operations (e.g., voting infrastructure and using [RFC
2777](https://tools.ietf.org/html/rfc2777)).

1.  Participants announce their candidacies. Participants have 14 days
    to announce their candidacies, but this period ends as soon as all
    participants have announced their intentions. If there is only one
    candidate, that person becomes the Chair. If there are two or more
    candidates, there is a vote. Otherwise, nothing changes.
2.  Participants vote. Participants have 21 days to vote for a single
    candidate, but this period ends as soon as all participants have
    voted. The individual who receives the most votes, no two from the
    same organisation, is elected chair. In case of a tie, RFC2777 is
    used to break the tie. An elected Chair may appoint co-Chairs.

Participants dissatisfied with the outcome of an election may ask the
Community Development Lead to intervene. The Community Development Lead,
after evaluating the election, may take any action including no action.

## Amendments to this Charter

The group can decide to work on a proposed amended charter, editing the
text using the [Decision Process](#decision-process) described above. The
decision on whether to adopt the amended charter is made by conducting a
30-day vote on the proposed new charter. The new charter, if approved,
takes effect on either the proposed date in the charter itself, or 7
days after the result of the election is announced, whichever is later.
A new charter must receive 2/3 of the votes cast in the approval vote to
pass. The group may make simple corrections to the charter such as
deliverable dates by the simpler group decision process rather than this
charter amendment process. The group will use the amendment process for
any substantive changes to the goals, scope, deliverables, decision
process or rules for amending the charter.

[library crates]: https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html
[RDF]: https://www.w3.org/TR/rdf-primer/
[Rust]: https://www.rust-lang.org/
[Turtle]: https://www.w3.org/TR/rdf-turtle/
[SHACL]: https://www.w3.org/TR/shacl/
[zero-cost abstraction]: https://blog.rust-lang.org/2015/05/11/traits.html
[wrapper type]: https://doc.rust-lang.org/book/ch20-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
[RDF Javascript Libraries Community Group]: https://www.w3.org/groups/cg/rdfjs/
[CODEOWNERS]: https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners#codeowners-and-branch-protection
[write access]: https://docs.github.com/en/get-started/learning-about-github/github-glossary#write-access
[pull-request hack]: https://felixge.de/2013/03/11/the-pull-request-hack/
[Section 5.2]: https://www.w3.org/policies/process/20231103/#consensus-building
[crates.io]: https://crates.io/
[Final Community Group Report]: 

