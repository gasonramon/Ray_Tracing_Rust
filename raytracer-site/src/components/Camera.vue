<template>
  <div>
    <h1>Camera</h1>
    <p>
      The camera maps 2D pixel coordinates to 3D rays. You give it a <code>lookfrom</code> and
      <code>lookat</code> point, a vertical field of view, an aspect ratio, and optional
      depth-of-field parameters, and it handles the rest.
    </p>

    <h2>Building the coordinate frame</h2>
    <p>
      Three orthonormal vectors are derived from the camera parameters using cross products.
      This gives us a local coordinate system to offset the viewport in 3D space.
    </p>
    <pre><code><span class="keyword">let</span> w <span class="punct">=</span> <span class="fn-name">unit_vector</span><span class="punct">(</span>lookfrom <span class="punct">-</span> lookat<span class="punct">);</span>  <span class="comment">// points backward (into camera)</span>
<span class="keyword">let</span> u <span class="punct">=</span> <span class="fn-name">unit_vector</span><span class="punct">(</span><span class="fn-name">cross</span><span class="punct">(</span>vup<span class="punct">,</span> w<span class="punct">));</span>   <span class="comment">// camera right</span>
<span class="keyword">let</span> v <span class="punct">=</span> <span class="fn-name">cross</span><span class="punct">(</span>w<span class="punct">,</span> u<span class="punct">);</span>                  <span class="comment">// camera up</span></code></pre>

    <h2>Field of view</h2>
    <p>
      Vertical FOV (in degrees) controls how wide the viewport is.
      A small angle compresses the scene like a telephoto lens; a large angle stretches it out.
    </p>
    <div class="math-block">
      h = tan(vfov / 2)<br/>
      viewport_height = 2h<br/>
      viewport_width = aspect_ratio × viewport_height
    </div>
    <pre><code><span class="keyword">let</span> theta            <span class="punct">=</span> vfov<span class="punct">.</span><span class="fn-name">to_radians</span><span class="punct">();</span>
<span class="keyword">let</span> h                <span class="punct">=</span> <span class="punct">(</span>theta <span class="punct">/</span> <span class="number">2.0</span><span class="punct">).</span><span class="fn-name">tan</span><span class="punct">();</span>
<span class="keyword">let</span> viewport_height  <span class="punct">=</span> <span class="number">2.0</span> <span class="punct">*</span> h<span class="punct">;</span>
<span class="keyword">let</span> viewport_width   <span class="punct">=</span> aspect_ratio <span class="punct">*</span> viewport_height<span class="punct">;</span></code></pre>

    <div class="fov-demo">
      <div v-for="fov in fovExamples" :key="fov.deg" class="fov-card">
        <div class="fov-vis" :style="{ '--angle': fov.deg + 'deg' }">
          <div class="fov-cone"></div>
        </div>
        <div class="fov-label">{{ fov.deg }}° — {{ fov.name }}</div>
      </div>
    </div>

    <h2>Viewport and lower-left corner</h2>
    <p>
      The viewport is a virtual rectangle in 3D space that the rays pass through.
      The lower-left corner is computed from the origin, horizontal/vertical extents,
      and the focus distance (which scales the viewport in depth-of-field mode).
    </p>
    <pre><code><span class="keyword">let</span> horizontal <span class="punct">=</span> focus_dist <span class="punct">*</span> viewport_width  <span class="punct">*</span> u<span class="punct">;</span>
<span class="keyword">let</span> vertical   <span class="punct">=</span> focus_dist <span class="punct">*</span> viewport_height <span class="punct">*</span> v<span class="punct">;</span>
<span class="keyword">let</span> lower_left_corner
    <span class="punct">=</span> origin
    <span class="punct">-</span> horizontal <span class="punct">/</span> <span class="number">2.0</span>
    <span class="punct">-</span> vertical   <span class="punct">/</span> <span class="number">2.0</span>
    <span class="punct">-</span> focus_dist <span class="punct">*</span> w<span class="punct">;</span></code></pre>

    <h2>Depth of field</h2>
    <p>
      Real cameras focus at one distance and blur objects closer or farther away.
      This is simulated with a <em>thin-lens</em> model: instead of firing rays from a single point,
      rays originate from random points inside a disk of radius <code>lens_radius</code>.
      All rays converge at the <code>focus_dist</code> plane, creating natural bokeh.
    </p>
    <div class="math-block">
      lens_radius = aperture / 2
    </div>
    <pre><code><span class="keyword">pub fn</span> <span class="fn-name">get_ray</span><span class="punct">(&</span><span class="keyword">self</span><span class="punct">,</span> s<span class="punct">:</span> <span class="type">f64</span><span class="punct">,</span> t<span class="punct">:</span> <span class="type">f64</span><span class="punct">) -></span> <span class="type">Ray</span> <span class="punct">{</span>
    <span class="keyword">let</span> rd     <span class="punct">=</span> <span class="keyword">self</span><span class="punct">.</span>lens_radius <span class="punct">*</span> <span class="fn-name">random_in_unit_disk</span><span class="punct">();</span>
    <span class="keyword">let</span> offset <span class="punct">=</span> <span class="keyword">self</span><span class="punct">.</span>u <span class="punct">*</span> rd<span class="punct">.</span><span class="fn-name">get_x</span><span class="punct">() +</span> <span class="keyword">self</span><span class="punct">.</span>v <span class="punct">*</span> rd<span class="punct">.</span><span class="fn-name">get_y</span><span class="punct">();</span>

    <span class="type">Ray</span><span class="punct">::</span><span class="fn-name">new</span><span class="punct">(</span>
        <span class="keyword">self</span><span class="punct">.</span>origin <span class="punct">+</span> offset<span class="punct">,</span>
        <span class="keyword">self</span><span class="punct">.</span>lower_left_corner
            <span class="punct">+</span> s <span class="punct">*</span> <span class="keyword">self</span><span class="punct">.</span>horizontal
            <span class="punct">+</span> t <span class="punct">*</span> <span class="keyword">self</span><span class="punct">.</span>vertical
            <span class="punct">-</span> <span class="keyword">self</span><span class="punct">.</span>origin
            <span class="punct">-</span> offset<span class="punct">,</span>
    <span class="punct">)</span>
<span class="punct">}</span></code></pre>

    <h2>Final scene camera settings</h2>
    <div class="card">
      <table class="settings-table">
        <tr v-for="s in settings" :key="s.param">
          <td><code>{{ s.param }}</code></td>
          <td class="val">{{ s.value }}</td>
          <td class="desc">{{ s.desc }}</td>
        </tr>
      </table>
    </div>

    <div class="nav-footer">
      <RouterLink to="/materials" class="prev-link">← Materials</RouterLink>
      <RouterLink to="/gallery" class="next-link">Gallery →</RouterLink>
    </div>
  </div>
</template>

<script setup>
const fovExamples = [
  { deg: 20,  name: 'Telephoto' },
  { deg: 50,  name: 'Normal' },
  { deg: 90,  name: 'Wide' },
]

const settings = [
  { param: 'lookfrom',    value: '(13, 2, 3)',  desc: 'Camera position' },
  { param: 'lookat',      value: '(0, 0, 0)',   desc: 'Target point' },
  { param: 'vup',         value: '(0, 1, 0)',   desc: 'World up vector' },
  { param: 'vfov',        value: '20°',         desc: 'Vertical field of view (telephoto)' },
  { param: 'aspect_ratio',value: '3:2',         desc: '1200 × 800 output' },
  { param: 'aperture',    value: '0.1',         desc: 'Slight depth of field' },
  { param: 'focus_dist',  value: '10.0',        desc: 'Scene focused at 10 units' },
]
</script>

<style scoped>
.fov-demo {
  display: flex; gap: 1.5rem; margin: 1.5rem 0; flex-wrap: wrap;
}
.fov-card {
  display: flex; flex-direction: column; align-items: center; gap: 0.6rem;
  background: var(--surface); border: 1px solid var(--border); border-radius: 10px;
  padding: 1.2rem 1.5rem;
}
.fov-vis {
  width: 80px; height: 60px; position: relative; display: flex; align-items: flex-end; justify-content: center;
}
.fov-cone {
  width: 0; height: 0;
  border-left: calc(tan(calc(var(--angle) / 2)) * 40px) solid transparent;
  border-right: calc(tan(calc(var(--angle) / 2)) * 40px) solid transparent;
  border-bottom: 40px solid #3a2a60;
}
.fov-label { font-size: 0.82rem; color: var(--muted); text-align: center; }

.settings-table { width: 100%; border-collapse: collapse; }
.settings-table tr { border-bottom: 1px solid var(--border); }
.settings-table tr:last-child { border-bottom: none; }
.settings-table td { padding: 0.6rem 0.5rem; font-size: 0.88rem; }
.settings-table td.val { color: var(--accent2); font-family: 'JetBrains Mono', monospace; width: 110px; }
.settings-table td.desc { color: var(--muted); }

.nav-footer { margin-top: 3rem; padding-top: 1.5rem; border-top: 1px solid var(--border); display: flex; gap: 1rem; }
.next-link, .prev-link { display: inline-flex; align-items: center; border-radius: 8px; padding: 0.6rem 1.2rem; font-weight: 600; font-size: 0.9rem; transition: opacity 0.15s; }
.next-link { background: var(--accent); color: #fff; }
.prev-link { background: var(--surface); border: 1px solid var(--border); color: var(--text); }
.next-link:hover, .prev-link:hover { opacity: 0.8; }
</style>
