<template>
  <div class="min-h-screen bg-slate-950 text-white">
    <div class="flex">
      <aside class="w-72 border-r border-slate-800 bg-slate-900 min-h-screen">
        <div class="p-6 border-b border-slate-800">
          <h1 class="text-2xl font-bold">Platform Admin</h1>
          <p class="text-slate-400 text-sm mt-1">
            ConsultOps Platform Controls
          </p>
        </div>

        <nav class="p-4 space-y-2">
          <NuxtLink
            to="/platform"
            class="block px-4 py-3 rounded-lg hover:bg-slate-800 transition"
          >
            Dashboard
          </NuxtLink>

          <NuxtLink
            to="/platform/users"
            class="block px-4 py-3 rounded-lg hover:bg-slate-800 transition"
          >
            Users
          </NuxtLink>

          <NuxtLink
            to="/platform/organizations"
            class="block px-4 py-3 rounded-lg hover:bg-slate-800 transition"
          >
            Organizations
          </NuxtLink>
        </nav>
      </aside>

      <main class="flex-1 p-8">
        <slot />
      </main>
    </div>
  </div>
</template>
