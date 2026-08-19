# Platform Compatibility

<p>
  Platform support per extension. <code>✓</code> = works, <code>✗</code> =
  not supported.
</p>

<table>
  <thead>
    <tr>
      <th>Extension</th>
      <th>Linux</th>
      <th>macOS</th>
      <th>Windows</th>
      <th>Notes</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><code>layout-override</code></td>
      <td>✓</td><td>✓</td><td>✓</td>
      <td>Pure JSON transformation, no OS dependencies.</td>
    </tr>
    <tr>
      <td><code>config-roulette</code></td>
      <td>✓</td><td>✓</td><td>✓</td>
      <td>Local file reads only; <code>~</code> expands via <code>HOME</code> with a <code>USERPROFILE</code> fallback on Windows.</td>
    </tr>
  </tbody>
</table>
