<template>
  <main class="landing-page">
    <section class="hero">
      <div class="hero-copy">
        <p class="eyebrow">Consult Ops</p>

        <h1>Turn client work into a trackable operational workflow.</h1>

        <p class="hero-text">
          Create engagements, generate contracts, track milestones, manage
          payments, notify every party, and close out work with a full audit
          trail.
        </p>

        <div class="hero-actions">
          <button class="primary" @click="$router.push('/register')">
            Start Free
          </button>

          <button class="secondary" @click="$router.push('/demo-dashboard')">
            View Demo
          </button>
        </div>
      </div>

      <div class="hero-panel">
        <div class="panel-header">
          <span>Engagement Power Track</span>
          <strong>82%</strong>
        </div>

        <div class="battery">
          <span class="charged" />
          <span class="charged" />
          <span class="charged" />
          <span class="charged" />
          <span />
        </div>

        <div class="mini-timeline">
          <p>✓ Contract sent</p>
          <p>✓ Activation paid</p>
          <p>✓ Milestone approved</p>
          <p>○ Final sign-off pending</p>
        </div>
      </div>
    </section>

    <section class="feature-grid">
      <article>
        <p class="eyebrow">Contracts</p>
        <h2>Generate agreements from real engagement data.</h2>
        <p>
          Convert scopes, deliverables, rates, parties, and milestones into an
          operational contract flow.
        </p>
      </article>

      <article>
        <p class="eyebrow">Milestones</p>
        <h2>Track work from submission to approval.</h2>
        <p>
          Submit, approve, reopen, and mark milestones paid while maintaining a
          full operational timeline.
        </p>
      </article>

      <article>
        <p class="eyebrow">Payments</p>
        <h2>Connect work completion to payment certainty.</h2>
        <p>
          Activation billing, payout rules, transaction records, and payment
          events keep everyone aligned.
        </p>
      </article>

      <article>
        <p class="eyebrow">Trust Layer</p>
        <h2>Every action leaves a trail.</h2>
        <p>
          Notifications, emails, timeline events, and status changes make the
          workflow visible and defensible.
        </p>
      </article>
    </section>

    <section class="use-cases">
      <div>
        <p class="eyebrow">Built For</p>
        <h2>Service work that needs structure.</h2>
      </div>

      <div class="use-case-list">
        <span>Freelance development</span>
        <span>Creative commissions</span>
        <span>Consulting retainers</span>
        <span>Agency projects</span>
        <span>Client onboarding</span>
        <span>Milestone-based payouts</span>
      </div>
    </section>

    <section class="pricing">
      <div class="pricing-card">
        <p class="eyebrow">Free</p>
        <h2>Starter</h2>
        <p class="price">$0</p>

        <ul>
          <li>Basic engagement tracking</li>
          <li>Contract preview generation</li>
          <li>Milestone timeline</li>
          <li>Manual payment tracking</li>
        </ul>

        <button class="secondary" @click="$router.push('/register')">
          Create Free Account
        </button>
      </div>

      <div class="pricing-card premium">
        <p class="eyebrow">Premium</p>
        <h2>Operations Pro</h2>
        <p class="price">$29/mo</p>

        <ul>
          <li>Multiple active engagements</li>
          <li>Agreement payout rules</li>
          <li>Stripe workflow visibility</li>
          <li>Email notifications</li>
          <li>Platform trust timeline</li>
        </ul>

        <button class="primary" @click="$router.push('/register')">
          Upgrade Workflow
        </button>
      </div>
    </section>

    <section class="final-cta">
      <p class="eyebrow">Operational Certainty</p>
      <h2>Stop managing client work from scattered messages.</h2>
      <p>
        Build the agreement, track the work, prove the payment path, and close
        the loop.
      </p>

      <button class="primary" @click="$router.push('/register')">
        Start Building Your Workspace
      </button>
    </section>
  </main>
</template>

<style scoped>
.landing-page {
  min-height: 100vh;
  padding: 64px clamp(20px, 5vw, 80px);
  background: radial-gradient(
      circle at 20% 0%,
      rgba(37, 99, 235, 0.22),
      transparent 35%
    ),
    radial-gradient(circle at 85% 10%, rgba(16, 185, 129, 0.2), transparent 35%),
    #020617;
  color: #e5eefc;
}

.hero {
  display: grid;
  grid-template-columns: minmax(0, 1.1fr) minmax(320px, 0.9fr);
  gap: 32px;
  align-items: center;
  margin-bottom: 32px;
}

.eyebrow {
  color: #67e8f9;
  font-size: 0.72rem;
  font-weight: 900;
  letter-spacing: 0.14em;
  text-transform: uppercase;
}

h1 {
  color: #f8fafc;
  font-size: clamp(3rem, 7vw, 6.5rem);
  line-height: 0.92;
  letter-spacing: -0.06em;
  margin: 0;
  max-width: 980px;
}

h2 {
  color: #f8fafc;
  line-height: 1.05;
  margin: 0 0 12px;
}

.hero-text {
  color: #cbd5e1;
  font-size: 1.15rem;
  line-height: 1.7;
  max-width: 680px;
  margin: 24px 0;
}

.hero-actions {
  display: flex;
  gap: 14px;
  flex-wrap: wrap;
}

button {
  border: 0;
  border-radius: 999px;
  cursor: pointer;
  font-weight: 900;
  padding: 14px 20px;
}

.primary {
  background: linear-gradient(90deg, #60a5fa, #34d399);
  color: #04111f;
  box-shadow: 0 18px 42px rgba(52, 211, 153, 0.18);
}

.secondary {
  border: 1px solid rgba(103, 232, 249, 0.28);
  background: rgba(15, 23, 42, 0.88);
  color: #cde7ff;
}

.hero-panel,
.feature-grid article,
.use-cases,
.pricing-card,
.final-cta {
  border: 1px solid rgba(45, 212, 191, 0.22);
  border-radius: 26px;
  background: radial-gradient(
      circle at top left,
      rgba(96, 165, 250, 0.13),
      transparent 34%
    ),
    linear-gradient(180deg, rgba(15, 23, 42, 0.96), rgba(2, 12, 23, 0.96));
  box-shadow: 0 24px 80px rgba(0, 0, 0, 0.28);
}

.hero-panel {
  padding: 26px;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  color: #f8fafc;
  font-weight: 900;
  margin-bottom: 18px;
}

.panel-header strong {
  color: #6ee7b7;
  font-size: 1.8rem;
}

.battery {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 7px;
  border: 2px solid rgba(148, 163, 184, 0.24);
  border-radius: 18px;
  padding: 8px;
  margin-bottom: 18px;
}

.battery span {
  height: 58px;
  transform: skewX(-10deg);
  border-radius: 8px;
  background: rgba(15, 23, 42, 0.88);
  border: 1px solid rgba(148, 163, 184, 0.18);
}

.battery .charged {
  background: linear-gradient(135deg, #60a5fa, #34d399);
  box-shadow: 0 0 28px rgba(52, 211, 153, 0.28);
}

.mini-timeline {
  display: grid;
  gap: 10px;
}

.mini-timeline p {
  margin: 0;
  color: #cbd5e1;
}

.feature-grid {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 18px;
  margin-bottom: 32px;
}

.feature-grid article {
  padding: 24px;
}

.feature-grid p,
.use-cases p,
.pricing-card li,
.final-cta p {
  color: #cbd5e1;
  line-height: 1.6;
}

.use-cases {
  display: grid;
  grid-template-columns: 0.8fr 1.2fr;
  gap: 24px;
  padding: 28px;
  margin-bottom: 32px;
}

.use-case-list {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.use-case-list span {
  border: 1px solid rgba(103, 232, 249, 0.22);
  border-radius: 999px;
  background: rgba(8, 31, 42, 0.78);
  color: #dbeafe;
  font-weight: 800;
  padding: 10px 13px;
}

.pricing {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 22px;
  margin-bottom: 32px;
}

.pricing-card {
  padding: 28px;
}

.pricing-card.premium {
  border-color: rgba(52, 211, 153, 0.44);
  box-shadow:
    0 0 44px rgba(52, 211, 153, 0.12),
    0 24px 80px rgba(0, 0, 0, 0.28);
}

.price {
  background: linear-gradient(90deg, #60a5fa, #34d399);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  font-size: 2.5rem;
  font-weight: 950;
  margin: 8px 0 14px;
}

ul {
  list-style: none;
  padding: 0;
  margin: 0 0 22px;
}

li {
  margin: 10px 0;
}

li::before {
  content: '✓';
  color: #34d399;
  font-weight: 900;
  margin-right: 8px;
}

.final-cta {
  text-align: center;
  padding: 42px;
}

.final-cta h2 {
  font-size: clamp(2rem, 4vw, 4rem);
  max-width: 900px;
  margin: 0 auto 16px;
}

.final-cta p {
  max-width: 680px;
  margin: 0 auto 24px;
}

@media (max-width: 980px) {
  .hero,
  .feature-grid,
  .use-cases,
  .pricing {
    grid-template-columns: 1fr;
  }

  h1 {
    font-size: clamp(3rem, 14vw, 5rem);
  }
}
</style>
