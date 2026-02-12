## 2025-02-18 - Accessibility: Replay Button
**Learning:** Adding a replay button to TTS messages is critical for accessibility. Users need to replay synthesized speech if they missed it.
**Action:** Always include playback controls (Replay/Stop) for TTS features. Ensure they are accessible (ARIA labels, keyboard nav).

## 2025-02-18 - Maintenance: Dead Code
**Learning:** `src/components/ui/stateful-button.tsx` was dead code that caused build failures due to missing `motion` dependency.
**Action:** Deleted the file instead of adding the dependency. Verify usage before fixing "broken" components.
