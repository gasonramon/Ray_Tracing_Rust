<template>
  <div class="layout">
    <nav class="sidebar">
      <div class="logo">
        <span class="logo-icon">◈</span>
        <span>Ray Tracer</span>
      </div>
      <ul class="nav-links">
        <li v-for="link in links" :key="link.to">
          <RouterLink :to="link.to" :class="{ active: $route.path === link.to }">
            <span class="nav-icon">{{ link.icon }}</span>
            {{ link.label }}
          </RouterLink>
        </li>
      </ul>
      <div class="sidebar-footer">
        <a href="https://github.com/jasonleijdekker/Ray_Tracing_Rust" target="_blank" class="github-link">
          ⌥ View on GitHub
        </a>
      </div>
    </nav>
    <main class="content">
      <RouterView />
    </main>
  </div>
</template>

<script setup>
const links = [
  { to: '/',          icon: '⌂', label: 'Overview' },
  { to: '/vectors',   icon: '→', label: 'Vectors & Math' },
  { to: '/rays',      icon: '⟶', label: 'Rays & Spheres' },
  { to: '/materials', icon: '◇', label: 'Materials' },
  { to: '/camera',    icon: '⊡', label: 'Camera' },
  { to: '/gallery',   icon: '◈', label: 'Gallery' },
]
</script>

<style scoped>
.layout {
  display: flex;
  min-height: 100vh;
}

.sidebar {
  width: 240px;
  flex-shrink: 0;
  background: var(--surface);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  padding: 1.5rem 1rem;
  position: sticky;
  top: 0;
  height: 100vh;
  overflow-y: auto;
}

.logo {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  font-size: 1.1rem;
  font-weight: 700;
  color: var(--text);
  padding: 0.5rem 0.75rem 1.5rem;
  border-bottom: 1px solid var(--border);
  margin-bottom: 1rem;
}

.logo-icon {
  color: var(--accent);
  font-size: 1.3rem;
}

.nav-links {
  list-style: none;
  flex: 1;
}

.nav-links li a {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  padding: 0.6rem 0.75rem;
  border-radius: 8px;
  color: var(--muted);
  font-size: 0.9rem;
  font-weight: 500;
  transition: background 0.15s, color 0.15s;
}

.nav-links li a:hover,
.nav-links li a.active {
  background: #1f1f30;
  color: var(--text);
}

.nav-links li a.active {
  color: var(--accent);
  background: #1a1535;
}

.nav-icon {
  font-size: 1rem;
  width: 1.2rem;
  text-align: center;
}

.sidebar-footer {
  padding-top: 1rem;
  border-top: 1px solid var(--border);
}

.github-link {
  display: block;
  padding: 0.6rem 0.75rem;
  font-size: 0.82rem;
  color: var(--muted);
  border-radius: 8px;
  transition: color 0.15s;
}

.github-link:hover { color: var(--text); }

.content {
  flex: 1;
  max-width: 860px;
  margin: 0 auto;
  padding: 3rem 2.5rem;
}

@media (max-width: 700px) {
  .layout { flex-direction: column; }
  .sidebar { width: 100%; height: auto; position: static; flex-direction: row; flex-wrap: wrap; padding: 1rem; }
  .logo { border-bottom: none; padding-bottom: 0; }
  .nav-links { display: flex; flex-wrap: wrap; }
  .sidebar-footer { display: none; }
  .content { padding: 1.5rem 1rem; }
}
</style>
