# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: auth.spec.ts >> Auth flow >> remember me creates a durable login session for seeded user
- Location: tests/e2e/auth.spec.ts:81:3

# Error details

```
Error: expect(received).toContain(expected) // indexOf

Matcher error: received value must not be null nor undefined

Received has value: null

Call Log:
- Timeout 5000ms exceeded while waiting on the predicate
```

# Page snapshot

```yaml
- generic [ref=e1]:
  - generic [ref=e3]:
    - banner [ref=e4]:
      - link "Consult Ops home" [ref=e5] [cursor=pointer]:
        - /url: /
        - img [ref=e7]
        - generic [ref=e31]:
          - strong [ref=e32]: Consult Ops
          - generic [ref=e33]: Operational certainty for service work
      - navigation [ref=e34]:
        - link "Register" [ref=e35] [cursor=pointer]:
          - /url: /register
        - link "Consultant Login" [ref=e36] [cursor=pointer]:
          - /url: /consultant-login
        - link "Client Login" [ref=e37] [cursor=pointer]:
          - /url: /client-login
        - link "Admin" [ref=e38] [cursor=pointer]:
          - /url: /admin-login
    - main [ref=e39]:
      - alert [ref=e40]
      - generic [ref=e41]:
        - generic [ref=e42]:
          - paragraph [ref=e43]: Consultant Workspace
          - heading "Run your client operations from one place." [level=1] [ref=e44]
          - paragraph [ref=e45]: Track clients, projects, contracts, invoices, and payments from a single operational dashboard.
        - generic [ref=e47]:
          - generic [ref=e48]:
            - paragraph [ref=e49]: Consult Ops
            - heading "Consultant Workspace Login" [level=2] [ref=e50]
            - paragraph [ref=e51]: Manage clients, projects, invoices, contracts, and payments.
          - generic [ref=e52]:
            - generic [ref=e53]: Email
            - textbox "Email" [ref=e54]:
              - /placeholder: Enter email
              - text: contractor@atlas.test
          - generic [ref=e55]:
            - generic [ref=e56]: Password
            - textbox "Password" [ref=e57]:
              - /placeholder: Enter password
              - text: DemoPass123!
          - generic [ref=e58]:
            - checkbox "Remember me" [checked] [ref=e59]
            - generic [ref=e60]: Remember me
          - link "Forgot password?" [ref=e61] [cursor=pointer]:
            - /url: /forgot-password
          - button "Enter Workspace" [active] [ref=e62] [cursor=pointer]
  - generic:
    - img
  - generic:
    - generic:
      - generic:
        - button "Go to parent" [disabled]
        - button "Open in editor"
        - button "Close"
  - generic [ref=e63]:
    - button "Toggle Nuxt DevTools" [ref=e64] [cursor=pointer]:
      - img [ref=e65]
    - generic "Page load time" [ref=e68]:
      - generic [ref=e69]: "495"
      - generic [ref=e70]: ms
    - button "Toggle Component Inspector" [ref=e72] [cursor=pointer]:
      - img [ref=e73]
```

# Test source

```ts
  5   | const seededConsultant = {
  6   |   email: 'contractor@atlas.test',
  7   |   password: 'DemoPass123!',
  8   | };
  9   | 
  10  | const uniqueEmail = () =>
  11  |   `auth-${Date.now()}-${Math.random().toString(36).slice(2)}@example.test`;
  12  | 
  13  | test.describe('Auth flow', () => {
  14  |   test('registers a user and requires email verification before login', async ({
  15  |     page,
  16  |   }) => {
  17  |     const email = uniqueEmail();
  18  | 
  19  |     await page.goto('/register');
  20  | 
  21  |     await page.getByLabel(/name/i).fill('Test User');
  22  |     await page.getByLabel(/email/i).fill(email);
  23  |     await page.getByLabel(/^Password$/i).fill('Password123!');
  24  |     await page.getByLabel(/^Confirm password$/i).fill('Password123!');
  25  | 
  26  |     await page
  27  |       .getByRole('button', { name: /register|create account|sign up/i })
  28  |       .click();
  29  | 
  30  |     await expect(
  31  |       page.getByText(
  32  |         /verify|verification|check your email|account created|registration/i,
  33  |       ),
  34  |     ).toBeVisible();
  35  | 
  36  |     await page.goto('/consultant-login');
  37  | 
  38  |     await page.getByLabel(/email/i).fill(email);
  39  |     await page.getByLabel(/^Password$/i).fill('Password123!');
  40  |     await page
  41  |       .getByRole('button', { name: /login|sign in|enter workspace/i })
  42  |       .click();
  43  | 
  44  |     await expect(
  45  |       page.getByText('Please verify your email before logging in.'),
  46  |     ).toBeVisible();
  47  |     await expect(
  48  |       page.getByRole('button', { name: /resend verification email/i }),
  49  |     ).toBeVisible();
  50  |   });
  51  | 
  52  |   test('logs in a verified seeded user and loads profile', async ({ page }) => {
  53  |     await page.goto('/consultant-login');
  54  | 
  55  |     await page.getByLabel(/email/i).fill(seededConsultant.email);
  56  |     await page.getByLabel(/^Password$/i).fill(seededConsultant.password);
  57  |     await page
  58  |       .getByRole('button', { name: /login|sign in|enter workspace/i })
  59  |       .click();
  60  | 
  61  |     await expect(page).not.toHaveURL(/consultant-login/);
  62  | 
  63  |     await page.goto('/profile');
  64  | 
  65  |     await expect(page.getByText(seededConsultant.email)).toBeVisible();
  66  |   });
  67  | 
  68  |   test('forgot password does not reveal whether an email exists', async ({
  69  |     page,
  70  |   }) => {
  71  |     await page.goto('/forgot-password');
  72  | 
  73  |     await page.getByLabel(/email/i).fill('missing-user@example.test');
  74  |     await page.getByRole('button', { name: /reset|send/i }).click();
  75  | 
  76  |     await expect(
  77  |       page.getByText(/if an account exists|password reset link has been sent/i),
  78  |     ).toBeVisible();
  79  |   });
  80  | 
  81  |   test('remember me creates a durable login session for seeded user', async ({
  82  |     page,
  83  |   }) => {
  84  |     await page.goto('/consultant-login');
  85  | 
  86  |     await page.getByLabel(/email/i).fill(seededConsultant.email);
  87  |     await page.getByLabel(/^Password$/i).fill(seededConsultant.password);
  88  | 
  89  |     const remember = page.getByLabel(/remember/i);
  90  |     await expect(remember).toBeVisible();
  91  |     await remember.check();
  92  | 
  93  |     await page
  94  |       .getByRole('button', { name: /login|sign in|enter workspace/i })
  95  |       .click();
  96  | 
  97  |     await expect
  98  |       .poll(async () =>
  99  |         page.evaluate(
  100 |           () =>
  101 |             window.localStorage.getItem('auth_user') ||
  102 |             window.localStorage.getItem('auth:user'),
  103 |         ),
  104 |       )
> 105 |       .toContain(seededConsultant.email);
      |        ^ Error: expect(received).toContain(expected) // indexOf
  106 | 
  107 |     await page.reload();
  108 | 
  109 |     await expect
  110 |       .poll(async () =>
  111 |         page.evaluate(
  112 |           () =>
  113 |             window.localStorage.getItem('auth_user') ||
  114 |             window.localStorage.getItem('auth:user'),
  115 |         ),
  116 |       )
  117 |       .toContain(seededConsultant.email);
  118 |   });
  119 | 
  120 |   test('user can view and revoke active sessions', async ({ page }) => {
  121 |     await page.goto('/consultant-login');
  122 | 
  123 |     await page.getByLabel(/email/i).fill(seededConsultant.email);
  124 |     await page.getByLabel(/^Password$/i).fill(seededConsultant.password);
  125 |     await page
  126 |       .getByRole('button', { name: /login|sign in|enter workspace/i })
  127 |       .click();
  128 | 
  129 |     await expect(page).not.toHaveURL(/consultant-login/);
  130 | 
  131 |     await page.goto('/profile');
  132 | 
  133 |     await expect(page.getByText(/active sessions|sessions/i)).toBeVisible();
  134 | 
  135 |     await page
  136 |       .getByRole('button', { name: /revoke|sign out session/i })
  137 |       .first()
  138 |       .click();
  139 | 
  140 |     await expect(page.getByText(/session revoked|signed out/i)).toBeVisible();
  141 |   });
  142 | });
  143 | 
```