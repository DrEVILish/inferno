# Upstream gap review and patch series

## Compared revisions

- Upstream: network-audio/inferno `dev`, `4aad648055e313bf89b681b911df39133a1de7aa`.
- Our fork: DrEVILish/inferno `dev`, `68560066bf3fad4711380d2688d44ff7240678a8`.
- Review date: 2026-11-15.

There are 43 non-merge fork commits outside upstream history. That is not a list of 43 missing features. The code has diverged, and upstream has newer Windows, macOS, PTPv1, multicast and stream-lifecycle work. This review compares behavior and preserves those upstream changes. It does not propose replacing upstream files with the fork's older versions.

No AGENTS.md, CONTRIBUTING.md, CODING_STANDARDS.md or GLOSSARY.md was found in the compared upstream tree. The scope is the user request to compare the forks and submit small upstream patches. Reviews checked the existing APIs, call sites, tests and preservation of newer upstream behavior.

## Submitted patches

| PR | Change |
| --- | --- |
| [#2](https://github.com/network-audio/inferno/pull/2) | Bounds-check ARC item arrays |
| [#3](https://github.com/network-audio/inferno/pull/3) | Write and sync saved state before renaming |
| [#4](https://github.com/network-audio/inferno/pull/4) | Ignore unrelated answers during mDNS address checks |
| [#5](https://github.com/network-audio/inferno/pull/5) | Parse flow requests before reading counts and offsets |
| [#6](https://github.com/network-audio/inferno/pull/6) | Validate TX flow layouts before starting transmission |
| [#7](https://github.com/network-audio/inferno/pull/7) | Advertise TX latency in channel and bundle mDNS records |
| [#8](https://github.com/network-audio/inferno/pull/8) | Avoid overflow when converting latency to samples |
| [#9](https://github.com/network-audio/inferno/pull/9) | Clamp string writes to the destination buffer |
| [#10](https://github.com/network-audio/inferno/pull/10) | Drop oversized responses before packet construction |
| [#11](https://github.com/network-audio/inferno/pull/11) | Remove every channel in a bulk unsubscribe request |
| [#12](https://github.com/network-audio/inferno/pull/12) | Answer TX flow label queries with an empty page |
| [#13](https://github.com/network-audio/inferno/pull/13) | Clear subscription readback before asynchronous teardown |
| [#14](https://github.com/network-audio/inferno/pull/14) | Bound remote audio formats and receive flow slots |
| [#15](https://github.com/network-audio/inferno/pull/15) | Allow a host to pin its saved-state directory |
| [#16](https://github.com/network-audio/inferno/pull/16) | Announce a configurable product version |
| [#17](https://github.com/network-audio/inferno/pull/17) | Pack ARC channel pages without unused table slots |

Each PR starts from upstream dev independently. Each includes a patch note with the original problem, changed behavior, observed tests and known limits. They can be reviewed and merged separately. PRs that touch the same source file may need a rebase after another merges.

## Test evidence

Every submitted branch passed `cargo test -p inferno_aoip --lib -- --test-threads=1` on Linux. The TX latency patch also passed a live loopback mDNS query for both channel and bundle records. The flow-label patch passed a running-device UDP test. The response-size patch checks an oversized response followed by a valid response at the exact buffer limit.

The initial parallel library run hit the existing PTP kernel receive-timestamp timing test. Serial branch runs passed. Tests do not prove operation on real Dante hardware, crash durability after a power cut, or Windows/macOS behavior. Upstream CI has portable and Docker jobs for that additional evidence. The older fork workspace's Searchfire timeout and stale `searchlight` doctests are not used as evidence about the current upstream submodule.

Review used separate protocol and safety comparisons, followed by reviews of the ported diffs. One state-directory concern prompted a regression test. Upstream already treats empty configuration values as unset; the port preserves that behavior and also ignores a directly supplied empty PathBuf. No user state directories or production network interfaces were changed by these tests.

## Initial upstream CI results

The first submitted PR has additional CI failures, so this series is not reported as fully green:

- [Linux job](https://github.com/network-audio/inferno/actions/runs/43736025321/job/132878700540): all 23 ALSA DAW tests fail at the plugin-path assertion. The test executable runs under `target/debug/build/alsa_pcm_inferno/.../out/`, while `plugin_path()` assumes `target/debug/deps/` and looks for the shared library in the wrong directory. The affected test helper is unchanged by the ARC decoder patch. A standalone test-harness fix is appropriate; no such fix is included here.
- [Windows shared-IP 44.1 kHz job](https://github.com/network-audio/inferno/actions/runs/43736025334/job/132878701325): the microphone path received matching audio, but the speaker recording reports wrong audio after about 1.6 seconds of matching frames. Other end-to-end variants passed. The cause is not established, and this review does not label it a pre-existing flake or claim hardware compatibility.
- At submission, most other jobs were queued or still running. Local serial library results remain separate from CI results.

## Confirmed gaps needing later increments

These are absent upstream and are not implemented by the submitted batch. They are listed separately so the series does not claim full convergence.

| Gap | Small next increment | Dependencies and required proof |
| --- | --- | --- |
| Multicast create/delete bounds | Parse descriptor offsets, complete two-byte channel IDs and delete headers before acting. | Reuse the TX layout checks once accepted. Add malformed-packet and valid multicast wire tests. Preserve shared multicast socket behavior. |
| Channel-name validation | Reject invalid saved and controller-supplied channel labels, and handle invalid DNS labels without panicking. | Keep this separate from fixed names. Test byte limits, separators, saved state and rename responses. |
| Fixed last-channel name | Add FIXED_LAST_CHANNEL_NAME, preserve it when loading state, and refuse controller renames of fixed channels. | Build on channel-name validation. Preserve upstream SettingsError and all channel constructors. |
| Unsupported bit depth at the flow endpoint | Reject the requested bit depth before converting it to bytes. | The TX layout PR validates byte widths, but division by eight alone cannot distinguish every invalid bit count. Add endpoint tests for 17/25-bit requests. |
| Bounded flow names | Bound RX hostname and flow-name strings in the checked parser. | Follow the flow-parser PR. Test the maximum accepted length and one byte beyond it. |
| ALSA FFI panic containment | Wrap current extern callbacks with panic-to-error handling. | Port to the current lifecycle rather than copying the fork's old ALSA implementation. Test error conversion and pointer queries before prepare. |
| MAX_RX_LATENCY_NS | Refuse subscriptions whose effective latency exceeds a configured receive limit. | Keep RX, TX and limit settings independent. The fork uses status 0x1a, whose interpretation remains unverified against hardware. Do not present the status as confirmed Dante behavior. |
| ARC device settings reads | Return implemented sample-rate and receive-latency properties instead of zero-filled placeholders. | Limit the directory to properties actually implemented. The fork's broad readable/writable property table is not sufficient evidence. Add response fixtures. |
| Host-owned device renames | Add validated NAME_REQUEST_PATH and the ARC write handler. | Specify host acknowledgement and error behavior. Test atomic request-file creation and refused writes. |
| Host-owned latency changes | Add LATENCY_REQUEST_PATH and the ARC property write handler. | Agree how pending and applied latency differ. Do not report a requested value as already active without host confirmation. |
| Host-owned sample-rate changes | Add SAMPLE_RATE_REQUEST_PATH and the ARC property write handler. | Requires host confirmation and consistent readback/advertisement behavior. The fork's immediate announced value is not proof that the device changed rate. |
| Unsupported opcode replies | Add explicit refusal replies for unknown ARC/CMC/DBCP operations. | Separate generic refusal from guessed identify/reset/upgrade opcodes. Test actual response codes over UDP. |

## Changes already covered or superseded upstream

- Sample-rate and encoding status probes already return requested equal to current with real supported-value lists. Upstream also answers pull-up, AES67, lock and codec status queries.
- Upstream clock status handles absent clock information and answers while undisciplined. Preserve its current leader and interface statistics behavior. The fork's hardcoded locked-follower response would be a regression.
- Upstream already validates ALT_PORT, truncates names at UTF-8 boundaries and returns startup errors. Do not replace these with the fork's older panic or silent-fallback paths.
- Upstream has stream reuse, transmitter/receiver buffer replacement, late-packet rejection, disconnect silence, shared multicast sockets and automatic ports. The fork's older DAW and restart patches must not replace these implementations.
- Transfer notification throttling has a different API upstream. The old four-per-period patch is not a direct missing feature and needs measurement before a separate change.

## Unverified or deliberately excluded fork code

- proto_conmon.rs has unused, unconfirmed constants and duplicate IDs. It is not a working ConMon implementation.
- Legacy CMC 0x1200 support redirects replies to embedded destinations without the promised Yamaha fixture. Confirm the protocol and destination rules before offering it upstream.
- Capability bits must not advertise runtime rate or encoding changes that the device cannot apply.
- The fork's u16 offset saturation can silently serialize an incorrect offset. The response cap and pagination fixes are preferable; do not port saturation.
- Reciprocal peak arithmetic and bitmask allocation rewrites are small optimizations, not demonstrated behavior gaps.
- The old request-server restart loop changes lifecycle and could hide repeated failures. Prefer checked parsers and current startup errors; a restart policy needs its own design and tests.
- Statime link-down work is in a separate repository and PR, https://github.com/teodly/statime/pull/1. Upstream Inferno now has its own PTPv1 implementation. Do not mix the Statime patch into this series.

## Fork commit ledger

| Fork commit | Original change | Disposition |
| --- | --- | --- |
| [`629a024`](https://github.com/DrEVILish/inferno/commit/629a024) | tests: verify TX latency in channel and bundle mDNS records | Ported in TX latency PR, adapted to current upstream. |
| [`e4f21ed`](https://github.com/DrEVILish/inferno/commit/e4f21ed) | tests: latency limit check, MAX_RX_LATENCY_NS / TX_LATENCY_NS settings | Partial. TX latency evidence ported; receive-limit tests stay with the unverified 0x1a follow-up. |
| [`787d25c`](https://github.com/DrEVILish/inferno/commit/787d25c) | subscriber: MAX_RX_LATENCY_NS and the channel latency status (0x1A); document TX_LATENCY_NS for small-buffer receivers | Open. Receive-limit setting and channel-latency refusal need a separate port and hardware status verification. |
| [`5df04c1`](https://github.com/DrEVILish/inferno/commit/5df04c1) | mdns: advertise TX latency, not RX latency, for transmit channels and bundles | Ported in TX latency PR. |
| [`cecf2f1`](https://github.com/DrEVILish/inferno/commit/cecf2f1) | Harden Inferno: refusal semantics, legacy 0x1200 plane, capability helpers, rate writes, tests | Partial. Latency arithmetic, bounded byte writes and response-size cap ported. Unused ConMon ids, speculative capability flags and legacy discovery are excluded. Runtime property writes remain open. |
| [`59db015`](https://github.com/DrEVILish/inferno/commit/59db015) | receive latency: settable from controllers, and actually the receive latency | Open. Host-owned latency writes and applied-versus-pending semantics need a separate port. |
| [`059d974`](https://github.com/DrEVILish/inferno/commit/059d974) | settings: FIXED_LAST_CHANNEL_NAME pins the last RX and TX channel name | Open. Fixed names require validation, saved-state handling and rename-refusal tests. |
| [`104761f`](https://github.com/DrEVILish/inferno/commit/104761f) | mdns: an answer for another name is not an existing A record | Ported in mDNS address-match PR. |
| [`07274f8`](https://github.com/DrEVILish/inferno/commit/07274f8) | state storage: write fully and fsync before the atomic rename | Ported in durable-state PR. |
| [`c5a2b91`](https://github.com/DrEVILish/inferno/commit/c5a2b91) | settings: refuse an ALT_PORT whose following ports overflow | Already covered by upstream ALT_PORT validation. |
| [`368d02c`](https://github.com/DrEVILish/inferno/commit/368d02c) | settings: truncate names on a character boundary | Already covered by upstream UTF-8-safe truncation. |
| [`50d8050`](https://github.com/DrEVILish/inferno/commit/50d8050) | flows control: reject flow requests with an unsupported bit depth | Open. Reject bit depth at the flow endpoint before dividing by eight. |
| [`af922cb`](https://github.com/DrEVILish/inferno/commit/af922cb) | flows control: bound request names; never panic on an oversized response | Partial. Oversized-response guard ported; bounded hostname/flow-name parsing remains open. |
| [`9c26258`](https://github.com/DrEVILish/inferno/commit/9c26258) | arc: a page cut at the size limit carries no orphaned entry data | Ported in packed-pages PR. |
| [`321e4e5`](https://github.com/DrEVILish/inferno/commit/321e4e5) | info mcast: announce sample rate and encoding status at startup | Already covered by upstream settings-status startup announcements. |
| [`426fa2f`](https://github.com/DrEVILish/inferno/commit/426fa2f) | device info: advertise sample rate and encoding configuration | Excluded until runtime changes are implemented. Do not advertise unsupported rate/encoding configuration. |
| [`5347f79`](https://github.com/DrEVILish/inferno/commit/5347f79) | info mcast: capability status reports requested = current, real lists | Already covered by upstream requested/current values and supported lists. |
| [`11b7e86`](https://github.com/DrEVILish/inferno/commit/11b7e86) | info mcast: clock status reports a locked follower (sync state 3) | Superseded. Preserve upstream dynamic clock status rather than hardcoding lock. |
| [`e1f2577`](https://github.com/DrEVILish/inferno/commit/e1f2577) | state: STATE_DIR setting to pin where saved state lives | Ported in state-directory PR with the current state_dir helper and settings rules preserved. |
| [`b935282`](https://github.com/DrEVILish/inferno/commit/b935282) | arc: accept device renames from a controller (0x1001), handed to the host | Open. Host-owned rename protocol and request-file behavior need a separate port. |
| [`e6310d6`](https://github.com/DrEVILish/inferno/commit/e6310d6) | arc: real device settings (0x1100) and property directory (0x1102) | Open. Implemented properties only; do not copy the fork directory permissions wholesale. |
| [`0785eb2`](https://github.com/DrEVILish/inferno/commit/0785eb2) | arc: answer the TX flow labels query (0x2204) | Ported in TX-flow-labels PR with running-device UDP evidence. |
| [`cc4e9e6`](https://github.com/DrEVILish/inferno/commit/cc4e9e6) | info mcast: clock status with per-port records (role: follower) | Superseded by upstream clock/leader/status implementation. |
| [`0b647e3`](https://github.com/DrEVILish/inferno/commit/0b647e3) | device info: announce a configurable Product Version | Ported in product-version PR with upstream configuration error handling. |
| [`3881fff`](https://github.com/DrEVILish/inferno/commit/3881fff) | device server: rate-limit transfer notifications to 4 per period | Needs measurement against upstream TransferNotifier API; not a direct port. |
| [`382dc90`](https://github.com/DrEVILish/inferno/commit/382dc90) | device server: keep TX flows across a transmitter restart | Superseded by current transmitter source replacement and persistent flow lifecycle. |
| [`b837e3d`](https://github.com/DrEVILish/inferno/commit/b837e3d) | alsa plugin: regression test for stale audio after disconnect (#41) | Historical regression. Current upstream lifecycle and capture tests cover a redesigned implementation; do not transplant the old shell test as proof. |
| [`d2c936f`](https://github.com/DrEVILish/inferno/commit/d2c936f) | rx: learn the start time before connecting channels (#41 endless replay) | Superseded by current StreamAnchor and buffer-validity handling. |
| [`66d67fa`](https://github.com/DrEVILish/inferno/commit/66d67fa) | rx: silence sinks still attached when a flow's socket is removed (#41) | Superseded by upstream removed-flow silence handling; preserved during RX port. |
| [`d6f04ab`](https://github.com/DrEVILish/inferno/commit/d6f04ab) | rx: write a disconnect's silence ahead of the reader (#41) | Superseded by upstream silence and late-packet handling. |
| [`d0521f0`](https://github.com/DrEVILish/inferno/commit/d0521f0) | subscriber: clear a removed subscription before answering (U1 readback) | Ported in immediate-unsubscribe-readback PR. |
| [`2af5592`](https://github.com/DrEVILish/inferno/commit/2af5592) | alsa plugin: never let a panic in a callback abort the host (#8) | Open. Panic containment must wrap current ALSA callbacks, not the old lifecycle. |
| [`20d639a`](https://github.com/DrEVILish/inferno/commit/20d639a) | alsa plugin: don't abort the host when pointer is queried before prepare (#8) | Needs a current ALSA pre-prepare regression before porting; implementation has changed. |
| [`b512c60`](https://github.com/DrEVILish/inferno/commit/b512c60) | info server: answer sample-rate and encoding probes (U2) | Already covered by upstream sample-rate/encoding probes. |
| [`96811a3`](https://github.com/DrEVILish/inferno/commit/96811a3) | arc: bulk unsubscribe (0x3014) removes every listed channel (U1) | Ported in bulk-unsubscribe PR without changing the newer teardown implementation. |
| [`dbd9570`](https://github.com/DrEVILish/inferno/commit/dbd9570) | arc: pack channel-list pages; receive pages of 16 (U13) | Ported in packed-pages PR; upstream idle-RX listing preserved. |
| [`06993a1`](https://github.com/DrEVILish/inferno/commit/06993a1) | device server: restart request servers that panic (#49) | Not ported. Parser bounds and explicit startup failures take precedence over an unreviewed restart policy. |
| [`3baa1dd`](https://github.com/DrEVILish/inferno/commit/3baa1dd) | info mcast: don't panic on a malformed clock-stats file (#49) | Superseded by upstream safe clock/leader parsing. |
| [`789d551`](https://github.com/DrEVILish/inferno/commit/789d551) | rx: validate advertised formats and bound receive flows (#49) | Ported in RX format/flow-slot PR; shared sockets and removed-flow cleanup preserved. |
| [`def859f`](https://github.com/DrEVILish/inferno/commit/def859f) | arc: bounds-check 0x3014, multicast create/delete and channel renames (#49) | Partial. Bulk unsubscribe ported separately. Multicast bounds and channel-name validation remain open. |
| [`d7de045`](https://github.com/DrEVILish/inferno/commit/d7de045) | flows tx: validate flow layout before it reaches the TX thread (#49) | Ported in TX-layout PR; current transmitter timing code preserved. |
| [`ce8583a`](https://github.com/DrEVILish/inferno/commit/ce8583a) | flows control: parse requests with bounds checks (malformed packets, #49) | Ported in checked flow-parser PR; current listener ownership preserved. |
| [`3203a4f`](https://github.com/DrEVILish/inferno/commit/3203a4f) | protocol: bounds-check deserialize_items (malformed ARC packets, #49) | Ported in ARC item-bounds PR. |

The old CONMON_IMPLEMENTATION_PLAN.md is a historical work plan. Its completion markers do not establish wire compatibility or runtime integration. Use the behavior and evidence distinctions in this review when describing the fork.
