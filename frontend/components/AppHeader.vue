<script setup lang="ts">
import { computed, ref, onMounted } from 'vue';
import { useAuth } from '../composables/useAuth';
import NotificationBell from './Notifications/NotificationBell.vue';

const { authUser, isLoggedIn, restoreAuth, logout } = useAuth();

const menuOpen = ref(false);

const displayName = computed(() => {
  return authUser.value?.name || authUser.value?.email || 'User';
});

const roleLabel = computed(() => {
  return authUser.value?.user_type || authUser.value?.role || '';
});

const canImpersonate = computed(() => {
  return ['admin', 'super_admin'].includes(
    authUser.value?.user_type || authUser.value?.role || '',
  );
});

onMounted(() => {
  restoreAuth();
});
</script>

<template>
  <header class="app-header">
    <NuxtLink to="/" class="brand" aria-label="Consult Ops home">
      <span class="brand-mark" aria-hidden="true">
        <svg
          viewBox="0 0 72 48"
          class="brand-svg"
          role="img"
          aria-label="consult ops"
        >
          <defs>
            <linearGradient id="chainGrad" x1="0" y1="0" x2="1" y2="1">
              <stop offset="0%" stop-color="#34d399" />
              <stop offset="52%" stop-color="#10b981" />
              <stop offset="100%" stop-color="#67e8f9" />
            </linearGradient>

            <linearGradient id="gearGrad" x1="0" y1="0" x2="1" y2="1">
              <stop offset="0%" stop-color="#60a5fa" />
              <stop offset="52%" stop-color="#2563eb" />
              <stop offset="100%" stop-color="#22d3ee" />
            </linearGradient>

            <filter id="logoGlow" x="-40%" y="-40%" width="180%" height="180%">
              <feDropShadow
                dx="0"
                dy="0"
                stdDeviation="1.6"
                flood-color="#22d3ee"
                flood-opacity="0.45"
              />
            </filter>
          </defs>

          <!-- subtle base -->
          <rect
            x="1.5"
            y="1.5"
            width="69"
            height="45"
            rx="14"
            fill="rgba(2, 12, 23, 0.72)"
            stroke="rgba(103, 232, 249, 0.35)"
            stroke-width="1.4"
          />

          <!-- O / cog behind-right -->
          <g transform="translate(43 24)" opacity="0.9" filter="url(#logoGlow)">
            <path
              d="
        M 0 -17
        L 3.1 -14.2
        L 7.2 -15.4
        L 8.5 -11.4
        L 12.7 -10.2
        L 11.8 -6
        L 15.3 -3.4
        L 12.9 0
        L 15.3 3.4
        L 11.8 6
        L 12.7 10.2
        L 8.5 11.4
        L 7.2 15.4
        L 3.1 14.2
        L 0 17
        L -3.1 14.2
        L -7.2 15.4
        L -8.5 11.4
        L -12.7 10.2
        L -11.8 6
        L -15.3 3.4
        L -12.9 0
        L -15.3 -3.4
        L -11.8 -6
        L -12.7 -10.2
        L -8.5 -11.4
        L -7.2 -15.4
        L -3.1 -14.2
        Z
      "
              fill="url(#gearGrad)"
              stroke="#93c5fd"
              stroke-width="1.4"
              stroke-linejoin="miter"
            />

            <circle
              r="8.2"
              fill="rgba(2, 12, 23, 0.96)"
              stroke="#7dd3fc"
              stroke-width="1.8"
            />

            <circle r="3.1" fill="#60a5fa" opacity="0.95" />
          </g>

          <!-- C / chain links foreground-left -->
          <g
            transform="translate(28 24)"
            fill="rgba(2, 12, 23, 0.96)"
            stroke="url(#chainGrad)"
            stroke-width="3"
            stroke-linecap="round"
            stroke-linejoin="round"
            filter="url(#logoGlow)"
          >
            <!-- 8 chain links along a Times-like C curve -->
            <ellipse
              cx="1"
              cy="-15.2"
              rx="5.6"
              ry="3.4"
              transform="rotate(12 1 -15.2)"
            />
            <ellipse
              cx="-7.3"
              cy="-12.2"
              rx="5.6"
              ry="3.4"
              transform="rotate(42 -7.3 -12.2)"
            />
            <ellipse
              cx="-12.9"
              cy="-5.9"
              rx="5.6"
              ry="3.4"
              transform="rotate(72 -12.9 -5.9)"
            />
            <ellipse
              cx="-14.1"
              cy="2.6"
              rx="5.6"
              ry="3.4"
              transform="rotate(96 -14.1 2.6)"
            />
            <ellipse
              cx="-10.4"
              cy="10.1"
              rx="5.6"
              ry="3.4"
              transform="rotate(128 -10.4 10.1)"
            />
            <ellipse
              cx="-2.8"
              cy="14.4"
              rx="5.6"
              ry="3.4"
              transform="rotate(160 -2.8 14.4)"
            />
            <ellipse
              cx="5.7"
              cy="13.8"
              rx="5.6"
              ry="3.4"
              transform="rotate(184 5.7 13.8)"
            />
            <ellipse
              cx="10.5"
              cy="8.5"
              rx="5.2"
              ry="3.2"
              transform="rotate(218 10.5 8.5)"
            />

            <!-- inner highlight to make links read as chain -->
            <g stroke="#a7f3d0" stroke-width="1.1" opacity="0.85">
              <ellipse
                cx="1"
                cy="-15.2"
                rx="2.45"
                ry="1.35"
                transform="rotate(12 1 -15.2)"
              />
              <ellipse
                cx="-7.3"
                cy="-12.2"
                rx="2.45"
                ry="1.35"
                transform="rotate(42 -7.3 -12.2)"
              />
              <ellipse
                cx="-12.9"
                cy="-5.9"
                rx="2.45"
                ry="1.35"
                transform="rotate(72 -12.9 -5.9)"
              />
              <ellipse
                cx="-14.1"
                cy="2.6"
                rx="2.45"
                ry="1.35"
                transform="rotate(96 -14.1 2.6)"
              />
              <ellipse
                cx="-10.4"
                cy="10.1"
                rx="2.45"
                ry="1.35"
                transform="rotate(128 -10.4 10.1)"
              />
              <ellipse
                cx="-2.8"
                cy="14.4"
                rx="2.45"
                ry="1.35"
                transform="rotate(160 -2.8 14.4)"
              />
              <ellipse
                cx="5.7"
                cy="13.8"
                rx="2.45"
                ry="1.35"
                transform="rotate(184 5.7 13.8)"
              />
              <ellipse
                cx="10.5"
                cy="8.5"
                rx="2.25"
                ry="1.25"
                transform="rotate(218 10.5 8.5)"
              />
            </g>
          </g>
        </svg>
      </span>

      <span class="brand-copy">
        <strong>Consult Ops</strong>
        <small>Operational certainty for service work</small>
      </span>
    </NuxtLink>

    <nav v-if="!isLoggedIn" class="public-nav">
      <NuxtLink to="/register">Register</NuxtLink>
      <NuxtLink to="/consultant-login">Consultant Login</NuxtLink>
      <NuxtLink to="/client-login">Client Login</NuxtLink>
      <NuxtLink to="/admin-login">Admin</NuxtLink>
    </nav>

    <div v-else class="header-actions">
      <NotificationBell />

      <div class="user-menu">
        <button class="user-button" type="button" @click="menuOpen = !menuOpen">
          <span class="avatar">
            {{ displayName.slice(0, 1).toUpperCase() }}
          </span>

          <span class="user-copy">
            <strong>{{ displayName }}</strong>
            <small>{{ roleLabel }}</small>
          </span>

          <span class="chevron">▾</span>
        </button>

        <div v-if="menuOpen" class="dropdown">
          <NuxtLink to="/engagements" @click="menuOpen = false">
            Engagements
          </NuxtLink>

          <NuxtLink to="/project-portal" @click="menuOpen = false">
            Dashboard
          </NuxtLink>

          <NuxtLink to="/settings/billing" @click="menuOpen = false">
            Billing
          </NuxtLink>

          <NuxtLink
            v-if="roleLabel === 'super_admin'"
            to="/platform"
            @click="menuOpen = false"
          >
            Platform Admin
          </NuxtLink>

          <button v-if="canImpersonate" type="button">Impersonate Role</button>

          <button
            type="button"
            class="danger"
            @click="
              async () => {
                menuOpen = false;
                await logout();
              }
            "
          >
            Logout
          </button>
        </div>
      </div>
    </div>
  </header>
</template>

<style scoped>
.app-header {
  position: sticky;
  top: 0;
  z-index: 40;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 2rem;
  padding: 0.95rem 2rem;
  border-bottom: 1px solid rgba(80, 210, 170, 0.25);
  background: radial-gradient(
      circle at 20% 0%,
      rgba(37, 99, 235, 0.28),
      transparent 32%
    ),
    radial-gradient(circle at 86% 0%, rgba(16, 185, 129, 0.22), transparent 34%),
    rgba(7, 17, 31, 0.94);
  backdrop-filter: blur(18px);
  color: white;
  box-shadow: 0 18px 70px rgba(0, 0, 0, 0.28);
}

.brand {
  display: flex;
  align-items: center;
  gap: 0.85rem;
  color: white;
  text-decoration: none;
}

.brand-mark {
  display: grid;
  place-items: center;
  width: 48px;
  height: 48px;
  border-radius: 18px;
  box-shadow:
    0 0 0 1px rgba(103, 232, 249, 0.22),
    0 16px 38px rgba(34, 211, 238, 0.12);
}

.brand-svg {
  width: 48px;
  height: 48px;
  display: block;
}

.brand-copy {
  display: grid;
  gap: 0.08rem;
}

.brand-copy strong {
  font-size: 1rem;
  letter-spacing: -0.02em;
}

small {
  display: block;
  color: #9fb3c8;
  font-size: 0.75rem;
}

.public-nav,
.header-actions {
  display: flex;
  align-items: center;
  gap: 1rem;
  flex-wrap: wrap;
}

.public-nav a {
  color: #cde7ff;
  text-decoration: none;
  font-size: 0.9rem;
}

.public-nav a:hover {
  color: #6ee7b7;
}

.user-menu {
  position: relative;
}

.user-button {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  border: 1px solid rgba(80, 210, 170, 0.35);
  border-radius: 999px;
  background:
    linear-gradient(rgba(15, 23, 42, 0.92), rgba(15, 23, 42, 0.92)) padding-box,
    linear-gradient(90deg, rgba(96, 165, 250, 0.8), rgba(52, 211, 153, 0.8))
      border-box;
  color: white;
  padding: 0.48rem 0.6rem 0.48rem 0.48rem;
  cursor: pointer;
  min-width: 210px;
}

.avatar {
  display: grid;
  place-items: center;
  width: 36px;
  height: 36px;
  border-radius: 999px;
  background: linear-gradient(135deg, #2563eb, #10b981);
  color: #f8fafc;
  font-weight: 900;
}

.user-copy {
  display: grid;
  flex: 1;
  text-align: left;
}

.user-copy strong {
  font-size: 0.86rem;
  line-height: 1.1;
}

.chevron {
  color: #67e8f9;
  font-size: 0.9rem;
}

.dropdown {
  position: absolute;
  right: 0;
  top: calc(100% + 0.65rem);
  min-width: 240px;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  padding: 0.55rem;
  border: 1px solid rgba(80, 210, 170, 0.35);
  border-radius: 1rem;
  background: radial-gradient(
      circle at top right,
      rgba(16, 185, 129, 0.16),
      transparent 36%
    ),
    #07111f;
  box-shadow: 0 20px 70px rgba(0, 0, 0, 0.45);
  z-index: 60;
}

.dropdown a,
.dropdown button {
  text-align: left;
  border: 0;
  background: transparent;
  color: #cde7ff;
  padding: 0.72rem;
  border-radius: 0.75rem;
  cursor: pointer;
  text-decoration: none;
  font: inherit;
}

.dropdown a:hover,
.dropdown button:hover {
  background: rgba(16, 185, 129, 0.12);
  color: #6ee7b7;
}

.dropdown .danger:hover {
  background: rgba(251, 113, 133, 0.14);
  color: #fecdd3;
}

@media (max-width: 760px) {
  .app-header {
    align-items: flex-start;
    flex-direction: column;
    padding: 1rem;
  }

  .header-actions,
  .public-nav {
    width: 100%;
    justify-content: space-between;
  }

  .user-button {
    min-width: 0;
  }
}
</style>
