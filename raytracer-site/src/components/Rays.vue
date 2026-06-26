<template>
  <div>
    <h1>Rays &amp; Spheres</h1>
    <p>A ray is the fundamental primitive of ray tracing — a point in space with a direction, parameterized by a scalar <code>t</code>.</p>

    <div class="result-preview">
      <img src="/ray-tracing-image-1.jpg" alt="Ray traced scene result" class="preview-img" />
      <p class="preview-caption">
        Every pixel in this image is the result of 500 rays being cast, bounced, and averaged — all
        computed by the math explained on this page.
      </p>
    </div>

    <h2>The Ray equation</h2>
    <p>Any point along a ray can be found by:</p>
    <div class="math-block">
      <strong>P</strong>(t) = <strong>origin</strong> + t · <strong>direction</strong>
    </div>
    <pre><code><span class="keyword">pub struct</span> <span class="type">Ray</span> <span class="punct">{</span>
    origin<span class="punct">:</span>    <span class="type">Point3</span><span class="punct">,</span>
    direction<span class="punct">:</span> <span class="type">Vec3</span><span class="punct">,</span>
<span class="punct">}</span>

<span class="keyword">pub fn</span> <span class="fn-name">get_position</span><span class="punct">(&</span><span class="keyword">self</span><span class="punct">,</span> t<span class="punct">:</span> <span class="type">f64</span><span class="punct">) -></span> <span class="type">Point3</span> <span class="punct">{</span>
    <span class="keyword">self</span><span class="punct">.</span>origin <span class="punct">+</span> <span class="keyword">self</span><span class="punct">.</span>direction <span class="punct">*</span> t
<span class="punct">}</span></code></pre>
    <p>When <code>t = 0</code> you're at the origin. Positive <code>t</code> moves forward along the ray — that's where intersections live. Negative <code>t</code> is behind the camera and is ignored.</p>

    <h2>Sphere intersection</h2>
    <p>
      A sphere centered at <strong>C</strong> with radius <em>r</em> satisfies:
    </p>
    <div class="math-block">
      |<strong>P</strong> − <strong>C</strong>|² = r²
    </div>
    <p>Substituting the ray equation P(t) and expanding gives a quadratic in t:</p>
    <div class="math-block">
      t²(<strong>d</strong>·<strong>d</strong>) + 2t(<strong>oc</strong>·<strong>d</strong>) + (<strong>oc</strong>·<strong>oc</strong> − r²) = 0
    </div>
    <p>where <strong>oc</strong> = <strong>origin</strong> − <strong>C</strong>. The discriminant tells us the number of solutions:</p>
    <div class="highlight-box">
      <div class="disc-row"><span class="disc-sym neg">Δ &lt; 0</span><span>No intersection — ray misses the sphere</span></div>
      <div class="disc-row"><span class="disc-sym zero">Δ = 0</span><span>One intersection — ray grazes the surface</span></div>
      <div class="disc-row"><span class="disc-sym pos">Δ &gt; 0</span><span>Two intersections — ray passes through</span></div>
    </div>

    <h2>The optimised hit test</h2>
    <p>
      Rather than the full <code>-b ± √Δ / 2a</code> form, the code uses the half-<code>b</code> trick.
      Because the middle term always has a factor of 2, we can substitute <code>h = b/2</code> and simplify.
      This saves two multiplications per intersection test across hundreds of thousands of rays.
    </p>
    <pre><code><span class="keyword">fn</span> <span class="fn-name">hit</span><span class="punct">(&</span><span class="keyword">self</span><span class="punct">,</span> ray<span class="punct">: &</span><span class="type">Ray</span><span class="punct">,</span> t_min<span class="punct">:</span> <span class="type">f64</span><span class="punct">,</span> t_max<span class="punct">:</span> <span class="type">f64</span><span class="punct">, ...) -></span> <span class="type">bool</span> <span class="punct">{</span>
    <span class="keyword">let</span> oc   <span class="punct">=</span> ray<span class="punct">.</span><span class="fn-name">get_origin</span><span class="punct">() -</span> <span class="keyword">self</span><span class="punct">.</span>center<span class="punct">;</span>
    <span class="keyword">let</span> a    <span class="punct">=</span> ray<span class="punct">.</span><span class="fn-name">get_direction</span><span class="punct">().</span><span class="fn-name">get_squared_length</span><span class="punct">();</span>
    <span class="keyword">let</span> halfb <span class="punct">=</span> <span class="fn-name">dot</span><span class="punct">(</span>oc<span class="punct">,</span> ray<span class="punct">.</span><span class="fn-name">get_direction</span><span class="punct">());</span>
    <span class="keyword">let</span> c    <span class="punct">=</span> oc<span class="punct">.</span><span class="fn-name">get_squared_length</span><span class="punct">() -</span> <span class="keyword">self</span><span class="punct">.</span>radius <span class="punct">*</span> <span class="keyword">self</span><span class="punct">.</span>radius<span class="punct">;</span>

    <span class="keyword">let</span> discriminant <span class="punct">=</span> halfb <span class="punct">*</span> halfb <span class="punct">-</span> a <span class="punct">*</span> c<span class="punct">;</span>
    <span class="keyword">if</span> discriminant <span class="punct">&lt;</span> <span class="number">0.0</span> <span class="punct">{</span> <span class="keyword">return false</span><span class="punct">; }</span>

    <span class="keyword">let</span> sqrt_d <span class="punct">=</span> discriminant<span class="punct">.</span><span class="fn-name">sqrt</span><span class="punct">();</span>
    <span class="keyword">let mut</span> root <span class="punct">= (-</span>halfb <span class="punct">-</span> sqrt_d<span class="punct">) /</span> a<span class="punct">;</span>
    <span class="comment">// try near root; fall back to far root if out of range</span>
    <span class="keyword">if</span> root <span class="punct">&lt;</span> t_min <span class="punct">||</span> root <span class="punct">&gt;</span> t_max <span class="punct">{</span>
        root <span class="punct">= (-</span>halfb <span class="punct">+</span> sqrt_d<span class="punct">) /</span> a<span class="punct">;</span>
        <span class="keyword">if</span> root <span class="punct">&lt;</span> t_min <span class="punct">||</span> root <span class="punct">&gt;</span> t_max <span class="punct">{</span> <span class="keyword">return false</span><span class="punct">; }</span>
    <span class="punct">}</span>
    <span class="comment">// ...</span>
    <span class="keyword">true</span>
<span class="punct">}</span></code></pre>

    <h2>Surface normals</h2>
    <p>
      A normal is a unit vector perpendicular to the surface at the hit point.
      For a sphere, it points from the center to the hit point, divided by the radius.
      The tracer always stores outward-facing normals and flips the sign depending on which face was hit,
      so shading math always works the same way.
    </p>
    <div class="math-block">
      <strong>n</strong> = (<strong>P</strong> − <strong>C</strong>) / r
    </div>

    <h2>The sky background</h2>
    <p>When a ray misses all objects it returns a sky color — a linear blend from white to blue based on the ray's Y component:</p>
    <pre><code><span class="keyword">let</span> unit_direction <span class="punct">=</span> <span class="fn-name">unit_vector</span><span class="punct">(</span>r<span class="punct">.</span><span class="fn-name">get_direction</span><span class="punct">());</span>
<span class="keyword">let</span> t <span class="punct">=</span> <span class="number">0.5</span> <span class="punct">*</span> <span class="punct">(</span>unit_direction<span class="punct">.</span><span class="fn-name">get_y</span><span class="punct">()</span> <span class="punct">+</span> <span class="number">1.0</span><span class="punct">);</span>
<span class="comment">// lerp: white at horizon (t=0), blue at top (t=1)</span>
<span class="punct">(</span><span class="number">1.0</span> <span class="punct">-</span> t<span class="punct">)</span> <span class="punct">*</span> <span class="type">Color</span><span class="punct">::</span><span class="fn-name">new</span><span class="punct">(</span><span class="number">1.0</span><span class="punct">,</span> <span class="number">1.0</span><span class="punct">,</span> <span class="number">1.0</span><span class="punct">)</span> <span class="punct">+</span> t <span class="punct">*</span> <span class="type">Color</span><span class="punct">::</span><span class="fn-name">new</span><span class="punct">(</span><span class="number">0.5</span><span class="punct">,</span> <span class="number">0.7</span><span class="punct">,</span> <span class="number">1.0</span><span class="punct">)</span></code></pre>

    <h2>Anti-aliasing via random sampling</h2>
    <p>
      Instead of shooting one ray per pixel, 500 rays are fired with random sub-pixel offsets.
      Their colors are averaged and gamma-corrected (√color) to produce smooth edges without aliasing.
    </p>
    <pre><code><span class="keyword">for</span> _ <span class="keyword">in</span> <span class="number">0</span><span class="punct">..</span>SAMPLES_PER_PIXEL <span class="punct">{</span>
    <span class="keyword">let</span> u <span class="punct">=</span> <span class="punct">(</span>j <span class="keyword">as</span> <span class="type">f64</span> <span class="punct">+</span> <span class="fn-name">random_double</span><span class="punct">()) / (</span>IMAGE_WIDTH  <span class="punct">-</span> <span class="number">1</span><span class="punct">)</span> <span class="keyword">as</span> <span class="type">f64</span><span class="punct">;</span>
    <span class="keyword">let</span> v <span class="punct">=</span> <span class="punct">(</span>i <span class="keyword">as</span> <span class="type">f64</span> <span class="punct">+</span> <span class="fn-name">random_double</span><span class="punct">()) / (</span>IMAGE_HEIGHT <span class="punct">-</span> <span class="number">1</span><span class="punct">)</span> <span class="keyword">as</span> <span class="type">f64</span><span class="punct">;</span>
    <span class="keyword">let</span> r <span class="punct">=</span> cam<span class="punct">.</span><span class="fn-name">get_ray</span><span class="punct">(</span>u<span class="punct">,</span> v<span class="punct">);</span>
    pixel_color <span class="punct">+=</span> <span class="fn-name">ray_color</span><span class="punct">(&</span>r<span class="punct">, &</span>world<span class="punct">,</span> MAX_DEPTH<span class="punct">);</span>
<span class="punct">}</span></code></pre>

    <div class="nav-footer">
      <RouterLink to="/vectors" class="prev-link">← Vectors</RouterLink>
      <RouterLink to="/materials" class="next-link">Next: Materials →</RouterLink>
    </div>
  </div>
</template>

<style scoped>
.result-preview {
  margin: 1.5rem 0 2rem;
  border: 1px solid var(--border);
  border-radius: 12px;
  overflow: hidden;
  background: #000;
}
.preview-img {
  width: 100%;
  display: block;
}
.preview-caption {
  padding: 0.9rem 1.2rem;
  background: var(--surface);
  border-top: 1px solid var(--border);
  font-size: 0.85rem;
  color: var(--muted);
  margin: 0;
}
.disc-row {
  display: flex; align-items: center; gap: 1rem; padding: 0.4rem 0;
  font-size: 0.9rem; color: var(--muted);
}
.disc-sym { font-family: 'JetBrains Mono', monospace; font-weight: 700; min-width: 60px; }
.disc-sym.neg  { color: #f38ba8; }
.disc-sym.zero { color: var(--accent); }
.disc-sym.pos  { color: var(--accent2); }
.nav-footer {
  margin-top: 3rem; padding-top: 1.5rem; border-top: 1px solid var(--border);
  display: flex; gap: 1rem;
}
.next-link, .prev-link {
  display: inline-flex; align-items: center; border-radius: 8px;
  padding: 0.6rem 1.2rem; font-weight: 600; font-size: 0.9rem; transition: opacity 0.15s;
}
.next-link { background: var(--accent); color: #fff; }
.prev-link { background: var(--surface); border: 1px solid var(--border); color: var(--text); }
.next-link:hover, .prev-link:hover { opacity: 0.8; }
</style>
