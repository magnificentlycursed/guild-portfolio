## NOTE: 

This feedback was written based on the content as of short commit [6781329](https://github.com/Navigators-Guild/apprentice-onboarding/tree/6781329c036e5d1d010b7309b3460a5e2ec98cac) and may not reflect the current content

### FORWARD.md

I like the learn by doing format. I like the mindset. The adversarial approach is philosophically good. My initial reaction to the 'brutal' and 'roast' framing is that this style of code review is a big tech hazing trope that chills marginalized folks advancement because they're not perceived as culture fits. I'll have farther feedback after I've used the reviewer, but my initial thought is the persona should be tough but fair like a strict instructor--the agent is providing hard feedback because it knows you are capable and is pushing you to improve.

### 00-foundations

#### 00-foundations/03-the-language-landscape.md

Prereq sequencing issues in 00-foundations/01-the-new-literacy.md. Logged GitHub issues https://github.com/Navigators-Guild/apprentice-onboarding/issues/3 and https://github.com/Navigators-Guild/apprentice-onboarding/issues/4. Will submit a PR: https://github.com/Navigators-Guild/apprentice-onboarding/blob/6781329c036e5d1d010b7309b3460a5e2ec98cac/00-foundations/03-the-language-landscape.md?plain=1#L114 and https://github.com/Navigators-Guild/apprentice-onboarding/blob/6781329c036e5d1d010b7309b3460a5e2ec98cac/00-foundations/03-the-language-landscape.md?plain=1#L138

Can include a .vscode folder with extensions.json in the project to prompt vscode to install the recommended extensions. This is a great way to achieve a baseline project standard for linting, formatting, etc: https://github.com/Navigators-Guild/apprentice-onboarding/blob/6781329c036e5d1d010b7309b3460a5e2ec98cac/00-foundations/04-your-workspace.md?plain=1#L21

https://tattoocoder.com/recommending-vscode-extensions-within-your-open-source-projects/

#### 00-foundations/05-git-just-enough.md

Some edge cases re: git. The git config --global tag is great if you've got one git identity. If you are managing multiple then setting git config user.name and git config user.email in a git initialized directory will override the global setting: https://github.com/Navigators-Guild/apprentice-onboarding/blob/6781329c036e5d1d010b7309b3460a5e2ec98cac/00-foundations/05-git-just-enough.md?plain=1#L23

Re GitHub. Your git config name and email and time zone are included in all commits. Privacy conscious users should enable GitHub's "Keep my email private option", "Block command line pushes that expose my email", and set to git config user.email to the generated address: https://github.com/Navigators-Guild/apprentice-onboarding/blob/6781329c036e5d1d010b7309b3460a5e2ec98cac/00-foundations/05-git-just-enough.md?plain=1#L34

https://docs.github.com/en/account-and-profile/how-tos/email-preferences/setting-your-commit-email-address

![[Pasted image 20260328202151.png]]

Recommend introducing git checkout -b or the newer git switch here as a soft introduction to git branching strategies. The commands I always show are

git add .
git checkout -b \<branch name\>
git commit -m \<commit message\>
git push

From there I'd show making PRs and merging in the browser: https://github.com/Navigators-Guild/apprentice-onboarding/blob/6781329c036e5d1d010b7309b3460a5e2ec98cac/00-foundations/05-git-just-enough.md?plain=1#L135

### 01-talking-to-agents

#### 01-talking-to-agents/01-how-agents-think.md

Cuneiform mentioned like you intended to setup an extended metaphor but the rest of the metaphor appears to have been removed or omitted: https://github.com/Navigators-Guild/apprentice-onboarding/blob/6781329c036e5d1d010b7309b3460a5e2ec98cac/01-talking-to-agents/01-how-agents-think.md?plain=1#L33

First mention of VDD uses an acronym. This should be expanded out and either defined here or have a note saying you will explain in depth in 02-the-methodology/01-how-we-build.md: https://github.com/Navigators-Guild/apprentice-onboarding/blob/6781329c036e5d1d010b7309b3460a5e2ec98cac/01-talking-to-agents/01-how-agents-think.md?plain=1#L51

#### 01-talking-to-agents/02-the-art-of-intent.md

The writing prompts section is great practical advice. If you made a quick reference sheet than this or similar should go on it: https://github.com/Navigators-Guild/apprentice-onboarding/blob/6781329c036e5d1d010b7309b3460a5e2ec98cac/01-talking-to-agents/02-the-art-of-intent.md?plain=1#L61

Folks who are familiar with Agile project management may benefit from a comparison to user stories which are often structured like **“As a \[persona\], I \[want to\], \[so that\].”** then expanded on. See https://www.atlassian.com/agile/project-management/user-stories for more information

### MISC

Suggest creating a GitHub issue template that has a reading checklist for each section and a link to guild project repos and portfolios. Tag it and assign a mentor to provide guidance and review