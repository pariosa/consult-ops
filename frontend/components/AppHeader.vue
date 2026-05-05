<script setup lang="ts">
const { authUser, isLoggedIn, logout } = useAuth();

const menuOpen = ref(false);

const displayName = computed(() => {
  return authUser.value?.name || authUser.value?.email || 'User';
});

const roleLabel = computed(() => {
  return authUser.value?.role || '';
});

const canImpersonate = computed(() => authUser.value?.role === 'admin');
</script>

<template>
  <header class="app-header">
    <NuxtLink to="/" class="brand">
      <span class="brand-mark">CO</span>
      <span>
        <strong>Consult Ops</strong>
        <small>Modern operations for service businesses</small>
      </span>
    </NuxtLink>

    <nav v-if="!isLoggedIn">
      <NuxtLink to="/register">Register</NuxtLink>
      <NuxtLink to="/consultant-login">Consultant Login</NuxtLink>
      <NuxtLink to="/client-login">Client Login</NuxtLink>
      <NuxtLink to="/admin-login">Admin</NuxtLink>
    </nav>

    <div v-else class="user-menu">
      <button class="user-button" @click="menuOpen = !menuOpen">
        <span>{{ displayName }}</span>
        <small>{{ roleLabel }}</small>
        <span>▾</span>
      </button>

      <div v-if="menuOpen" class="dropdown">
        <NuxtLink to="/project-portal">Dashboard</NuxtLink>
        <NuxtLink to="/settings/billing">Billing</NuxtLink>

        <button v-if="canImpersonate" type="button">Impersonate Role</button>

        <button type="button" @click="async () => await logout()">
          Logout
        </button>
      </div>
    </div>
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

.user-menu {
  position: relative;
}

.user-button {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  border: 1px solid rgba(80, 210, 170, 0.35);
  border-radius: 0.85rem;
  background: rgba(15, 23, 42, 0.9);
  color: white;
  padding: 0.65rem 0.9rem;
  cursor: pointer;
}

.dropdown {
  position: absolute;
  right: 0;
  top: calc(100% + 0.5rem);
  min-width: 220px;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  padding: 0.5rem;
  border: 1px solid rgba(80, 210, 170, 0.35);
  border-radius: 0.85rem;
  background: #07111f;
  box-shadow: 0 20px 70px rgba(0, 0, 0, 0.45);
  z-index: 50;
}

.dropdown a,
.dropdown button {
  text-align: left;
  border: 0;
  background: transparent;
  color: #cde7ff;
  padding: 0.7rem;
  border-radius: 0.6rem;
  cursor: pointer;
  text-decoration: none;
}

.dropdown a:hover,
.dropdown button:hover {
  background: rgba(16, 185, 129, 0.12);
  color: #6ee7b7;
}
</style>
