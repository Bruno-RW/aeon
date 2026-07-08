# Aeon

Aeon is a system-performance monitoring application for end users who want to understand how software usage affects their computer. This context exists to define the product language clearly before implementation details harden around the wrong concepts.

## Language

**User**:
A person using Aeon to understand how their computer and applications affect system performance.
_Avoid_: Operator, analyst, admin

**Application Usage**:
The observed activity of a user-facing application over time, as recorded by Aeon.
_Avoid_: Productivity, focus time, habit tracking

**Application**:
A user-facing program shown by Aeon, which may represent one process or a grouped set of related processes when that better matches how users understand the software.
_Avoid_: Raw process, subprocess tree

**System Performance**:
The responsiveness and resource behavior of the user's computer, especially CPU, RAM, and GPU usage as affected by running applications.
_Avoid_: Personal productivity, focus, efficiency

**Resource Usage**:
The measured consumption of a machine resource by the system or by an application over time.
_Avoid_: Performance score, benchmark

**System-wide Usage**:
Resource usage measured across the entire computer, regardless of which application is responsible.
_Avoid_: Per-app usage, process detail

**Per-application Usage**:
Resource usage attributed to a specific running application as presented to the user for diagnosis.
_Avoid_: System-wide usage, benchmark result

**Resource Hog**:
An application whose CPU, RAM, or GPU usage is high enough to be a likely cause of current machine pressure.
_Avoid_: Benchmark winner, malware, productivity distraction

**Live Monitoring**:
The real-time display of current resource usage as it changes.
_Avoid_: Historical analysis, benchmark run

**Short-term History**:
A recent time window of recorded resource usage used to explain what just happened during the current Aeon session.
_Avoid_: Long-term analytics, archival reporting

**Session History**:
In-memory recorded resource usage that exists only while the current Aeon session is open.
_Avoid_: Saved history, telemetry archive

**Performance Monitoring**:
The act of collecting and presenting information about application activity and its relationship to system performance.
_Avoid_: Time tracking, employee monitoring, parental control

**MVP**:
The first usable version of Aeon, focused on Windows-only, read-only monitoring of CPU, RAM, and GPU usage.
_Avoid_: Full product, cross-platform release

**Read-only Monitoring**:
A mode where Aeon observes and presents performance information without changing system state or controlling applications.
_Avoid_: Optimization, task control, system tuning

**Refresh Rate**:
The configured interval at which Aeon updates its live resource measurements and visualizations using fixed presets, including a paused state.
_Avoid_: Benchmark duration, retention period

**Paused Monitoring**:
A state where live sampling is temporarily stopped while the current dashboard view remains available for inspection.
_Avoid_: App exit, historical recording mode

**Mini History Chart**:
A compact visualization that shows the recent session-local change in a resource over time for a single application.
_Avoid_: Long-term report, benchmark graph

**Dashboard**:
The single main screen in Aeon that combines system-wide status and the per-application list for diagnosis.
_Avoid_: Multi-view workspace, customizable console

**Ranking Resource**:
The resource metric currently used to sort the application list, such as CPU, RAM, or GPU.
_Avoid_: Hidden pressure score, opaque ranking

**Application Detail Panel**:
A side panel in the dashboard that shows current resource metrics, a larger recent chart, and basic application metadata for the selected application without leaving the main diagnostic view.
_Avoid_: Separate detail page, modal workflow

**Application Metadata**:
Basic identifying information about an application, such as its display name, executable path, and process count.
_Avoid_: Deep process breakdown, control surface

**Alert**:
A threshold-based warning that proactively calls attention to a resource condition.
_Avoid_: Dashboard observation, background notification

**GPU Usage**:
The measured utilization of the graphics processor for the system or an application.
_Avoid_: CPU usage, frame rate

**GPU Memory**:
The measured graphics-memory consumption associated with the system or an application.
_Avoid_: System RAM, disk cache

**Temperature**:
A hardware sensor reading that may be shown by Aeon when the operating system and device expose it reliably.
_Avoid_: Guaranteed metric, synthetic estimate

**System Summary**:
The top section of the dashboard that presents the machine-wide state using current values and recent mini charts for key resources.
_Avoid_: App detail view, notification center
