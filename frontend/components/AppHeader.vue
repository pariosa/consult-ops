<template>
  <header class="app-header">
    <NuxtLink to="/" class="brand">
      <span class="brand-mark">CO</span>
      <span>
        <strong>Consult Ops</strong>
        <small>Modern operations for service businesses</small>
      </span>
    </NuxtLink>

    <nav>
      <NuxtLink to="/register">Register</NuxtLink>
      <NuxtLink to="/consultant-login">Consultant Login</NuxtLink>
      <NuxtLink to="/client-login">Client Login</NuxtLink>
      <NuxtLink to="/admin-login">Admin</NuxtLink>
    </nav>
  </header>
</template>

<style scoped>
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 2rem;
  padding: 1rem 2rem;
  border-bottom: 1px solid rgba(80, 210, 170, 0.25);
  background:
    linear-gradient(#07111f, #07111f) padding-box,
    linear-gradient(90deg, #2563eb, #10b981) border-box;
  color: white;
}

.brand {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  color: white;
  text-decoration: none;
}

.brand-mark {
  display: grid;
  place-items: center;
  width: 42px;
  height: 42px;
  border-radius: 0.9rem;
  background: linear-gradient(135deg, #2563eb, #10b981);
  font-weight: 800;
}

small {
  display: block;
  color: #9fb3c8;
  font-size: 0.75rem;
}

nav {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
}

nav a {
  color: #cde7ff;
  text-decoration: none;
  font-size: 0.9rem;
}

nav a:hover {
  color: #6ee7b7;
}
</style>
