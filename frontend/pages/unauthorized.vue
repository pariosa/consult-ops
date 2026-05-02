<template>
  <section class="unauthorized">
    <p class="eyebrow">Access Restricted</p>
    <h1>You do not have permission to view this page.</h1>
    <p>Please return to the correct portal for your account type.</p>

    <div class="actions">
      <NuxtLink to="/consultant-login">Consultant Login</NuxtLink>
      <NuxtLink to="/client-login">Client Login</NuxtLink>
      <NuxtLink to="/admin-login">Admin Login</NuxtLink>
    </div>
  </section>
</template>

<style scoped>
.unauthorized {
  max-width: 760px;
  margin: 5rem auto;
  padding: 2rem;
  color: white;
  border: 1px solid rgba(80, 210, 170, 0.25);
  border-radius: 1.25rem;
  background: rgba(8, 19, 31, 0.82);
}

.eyebrow {
  color: #55d6be;
  text-transform: uppercase;
  letter-spacing: 0.12em;
}

h1 {
  font-size: clamp(2rem, 5vw, 4rem);
  line-height: 1;
}

p {
  color: #a8bdd2;
}

.actions {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
  margin-top: 2rem;
}

a {
  color: #6ee7b7;
}
</style>
