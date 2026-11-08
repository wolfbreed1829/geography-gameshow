# YouTube video discovery

Reviewed November 8, 2026. Public program inventory: [PROGRAM-VIDEOS.json](PROGRAM-VIDEOS.json). Nine verified program videos: eight Geography, one Art. No Anatomy video was identified. This is a search snapshot, not a promise that every public or unlisted recording has been found.

## Sources and method

The user supplied the official [GeographyGameshow channel](https://www.youtube.com/@GeographyGameshow) and four public video links through Teams. The existing website already linked the official introduction. Exact YouTube oEmbed metadata and public watch-page `ytInitialPlayerResponse` metadata were retrieved, including titles, channel identity, duration, publication date, `playabilityStatus.status = OK`, and `playableInEmbed = true` for every included video.

The official channel is `UCPOMJlMUDeWYR7OfFdQ5lPA`. Its Videos tab contains four uploads, with no further result continuation, and its Shorts tab contains one short. All five were reviewed for relevance. The Wellfleet Recreation channel Videos tab contained eight uploads, only one about Neal. YouTube searches for Neal Nichols, Neal Nichols Jr, Geography Man Neal Nichols, and Neal Nichols drawing lesson found an additional classroom recording. Broad web searches mostly returned unrelated people or subjects; those were excluded.

## Included program videos

| Program | Video | Channel | Display duration |
| --- | --- | --- | --- |
| Geography | [Geography Game Show](https://www.youtube.com/watch?v=YnSI6XrQm8E) | GeographyGameshow | 5:37 |
| Geography | [Geography Game Show Part 1](https://www.youtube.com/watch?v=SKuUkcGcrZU) | GeographyGameshow | 9:58 |
| Geography | [Geography Game Show Part 2](https://www.youtube.com/watch?v=MEHKqdRsNSo) | GeographyGameshow | 9:46 |
| Geography | [May 3, 2025](https://www.youtube.com/watch?v=qvTMn18PzBU) | GeographyGameshow | 0:26 |
| Geography | [Drawing the U.S.A.](https://www.youtube.com/watch?v=koim6jITp2Y) | Marlon D Hester Sr. | 3:02 |
| Geography | [Geography Man — drawing demonstration](https://www.youtube.com/watch?v=h3m5YX98ldQ) | Cuentamesister | 0:56 |
| Geography | [Geography Man — classroom moment](https://www.youtube.com/watch?v=1h0yqsXz9s0) | Cuentamesister | 1:14 |
| Geography | [Geography Gameshow in Wellfleet](https://www.youtube.com/watch?v=zlDTOERUIiU) | MsPhoxy | 1:39 |
| Art | [Neal Nichols Jr. Drawing Show](https://www.youtube.com/watch?v=mVRWc0QSrxc) | Wellfleet Recreation | 57:09 |

Friendly display titles distinguish the two identically titled Geography Man clips. The original titles remain in the manifest. Display durations use YouTube's displayed rounding where available; integer `duration_seconds` comes from the public player metadata and may be one second shorter.

The May 2025 short visually shows Neal drawing a map; its date-only original title is retained in the manifest. The Art recording was sampled using YouTube storyboard frames from the opening, roughly 17–20 minutes, 34–37 minutes, and 50–54 minutes. These show Neal teaching at an easel, developing architectural drawings and a detailed building study. Its YouTube thumbnail is a classroom photograph, not representative of the studio lesson itself.

## Exclusions and privacy observations

- [Helicopter](https://www.youtube.com/watch?v=miaeYeP-SDc), 0:15, is an official-channel travel clip showing a helicopter in snowy Wales, Alaska. Its description confirms this. It is not a Geography Gameshow or art lesson and is excluded from the program library.
- Search results for Neal Nichols musicians, clergy, other artists, unrelated Geography Man presenters, sports, and generic geography quizzes are excluded. The Mrs. Thomas classroom blog explicitly says its embedded video is a different artist and is excluded as well.
- Visible children's faces appear in the official Parts 1 and 2 thumbnails. The Art video thumbnail includes a classroom audience with some side profiles. These videos are not certified to satisfy the earlier no-visible-children's-faces requirement for selected photos. The video request specifically supplies original YouTube recordings; the implementation uses those originals and illustrated covers, without adding the research thumbnails to the site. The separate photo galleries retain their existing face exclusions. Thumbnails and sampled frames are evidence for research, not a full frame-by-frame privacy audit.
- Embedded availability is a point-in-time check. YouTube uploaders can later change privacy, embedding settings, or delete a video; the site should provide a usable unavailable-video fallback.

Raw research responses, thumbnails, and analysis contact sheets are in the ignored `.playwright-mcp/video-research` directory. Public media must use YouTube's embedded player; source videos were not downloaded or rehosted.

## Implementation

Each program has Overview, Photos, and Videos navigation. Separate static routes keep overviews uncluttered and support direct links and refresh. Geography and Art each use one player with a searchable catalog. Anatomy has no invented video: its page links to the available drawing work and other programs.

No YouTube iframe loads until a visitor presses play or selects a recording. The iframe uses `youtube-nocookie.com`, inline mobile playback, native captions/fullscreen/keyboard controls, same-channel related-video preference, and `strict-origin-when-cross-origin` referrers. A direct YouTube fallback sits beneath the player. Selecting another video replaces the existing iframe source so recordings cannot overlap. The implementation follows [YouTube's official embed documentation](https://developers.google.com/youtube/player_parameters); it does not hide YouTube controls, promise ad-free playback, or download video files.

`scripts/generate-video-data.py` validates IDs and sources and generates `src/video_data.rs` from the manifest; `--check` detects drift. Browser tests exercise every selection using an intercepted provider response, while real-player smoke checks separately test provider loading and playback availability.

Validation: Rust release build emits 17 static routes; format and manifest checks pass. Browser checks covered all nine selections, direct routes, search and its empty state, keyboard activation, source replacement, minimum player dimensions, mobile layouts, and all 60 existing gallery photos. Separate real YouTube smoke checks confirmed advancing video time for the Geography introduction and the Art drawing lesson. The first parallel browser run encountered local system memory pressure; the affected checks passed sequentially.
