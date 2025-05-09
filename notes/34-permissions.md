Lecture Notes: 34 Permissions · Classes w/ Prof. Nat Tuck



[↓Skip to main content](#main-content)

[Classes w/ Prof. Nat Tuck](/)

* [Home](/)
* [cs2370](/classes/2025-01/cs2370/)
* [cs2470](/classes/2025-01/cs2470/)
* [cs4310](/classes/2025-01/cs4310/)
* [Inkfish](https://inkfish.homework.quest/)

Lecture Notes: 34 Permissions
=============================

2025 Apr 16·Updated: 2025 Apr 17·1 min

Finish Auth & Access Control:

<https://homework.quest/classes/2025-01/cs4310/christo-slides/12_Auth_and_Access.pptx>

TOTP 2FA:

* Generates secure 6-digit numeric 1 time passwords.
* Open standard, so it’s broadly supported and allows users to
  use their choice of app.
* Much nicer than emailing or texting a code.
* This can be applied to OS login, even locally.

Overflow: Passkeys

* Cryptographic keys are a really nice authentication method:
  + Very secure - generally impossible to brute force.
  + No need to type a password.
* Adoption has been slow. Two major issues:
  + Nobody had put the effort into good UX.
  + They typically allow for unattended authentication, which drives
    obnoxious IT security people nuts.
* Standard for SSH for years.
  + Linux/Unix remote logins.
  + Git server auth.
* “Passkeys” allow for web / app authentication with cryptographic keys.
  + Finally put in the UX effort, wide support in browsers and mobile
    app dev tools.
  + The standard requires a presence check, typically fingerprint
    or face scan.
  + Still some adoption and UX work to be done, but we should see
    much more of this going forward.
  + Problem: Losing keys and device sync.

![Nat Tuck](/img/author.jpg)

Author

Nat Tuck

---

[←→

Lecture Notes: 33 Auth and Access Control

2025 Apr 14](/classes/2025-01/cs4310/notes/33-auth-access/)

[Lecture Notes: 35 Concurrency

2025 Apr 17
→←](/classes/2025-01/cs4310/notes/35-concurrency/)

[↑](#the-top "Scroll to top")

©
2025
Nat Tuck

Powered by [Hugo](https://gohugo.io/) & [Congo](https://github.com/jpanther/congo)