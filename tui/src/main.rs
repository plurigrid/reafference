use std::{error::Error, io, time::{Duration, Instant}};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span, Text},
    widgets::{
        Axis, Block, Borders, Cell, Chart, Dataset, Gauge, GraphType, List, ListItem, ListState,
        Paragraph, Row, Sparkline, Table, TableState, Tabs, Wrap,
    },
    Frame, Terminal,
};

const SCALES: [&str; 7] = [
    "Whole Earth",
    "Population",
    "Cohort",
    "Site",
    "Patient",
    "Tissue",
    "Cell",
];

#[derive(Clone, Copy)]
enum FocusPane {
    Network,
    Queue,
    Detail,
}

impl FocusPane {
    fn next(self) -> Self {
        match self {
            Self::Network => Self::Queue,
            Self::Queue => Self::Detail,
            Self::Detail => Self::Network,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Network => "network",
            Self::Queue => "queue",
            Self::Detail => "detail",
        }
    }
}

#[derive(Clone)]
struct NetworkNode {
    name: &'static str,
    layer: &'static str,
    focus: &'static str,
    status: &'static str,
    coverage: u16,
    readiness: u16,
}

#[derive(Clone)]
struct WorkItem {
    title: &'static str,
    layer: &'static str,
    owner: &'static str,
    priority: &'static str,
    summary: &'static str,
}

struct App {
    scale_index: usize,
    focus: FocusPane,
    selected_network: usize,
    selected_queue: usize,
    tick: u64,
    last_refresh: Instant,
    status: String,
    network_nodes: Vec<NetworkNode>,
    work_queue: Vec<WorkItem>,
    throughput: Vec<u64>,
    signal_curve: Vec<(f64, f64)>,
    sync_ratio: f64,
    evidence_ratio: f64,
}

impl App {
    fn new() -> Self {
        Self {
            scale_index: 0,
            focus: FocusPane::Network,
            selected_network: 0,
            selected_queue: 0,
            tick: 0,
            last_refresh: Instant::now(),
            status: "MHIN walking skeleton ready".into(),
            network_nodes: vec![
                NetworkNode {
                    name: "Global observatory mesh",
                    layer: "whole-earth",
                    focus: "Sentinel climate and care covariates",
                    status: "nominal",
                    coverage: 82,
                    readiness: 74,
                },
                NetworkNode {
                    name: "Maternal outcomes federation",
                    layer: "population",
                    focus: "Regional risk stratification",
                    status: "warming",
                    coverage: 76,
                    readiness: 68,
                },
                NetworkNode {
                    name: "Inflammation responder cohort",
                    layer: "cohort",
                    focus: "Responder segmentation",
                    status: "nominal",
                    coverage: 71,
                    readiness: 81,
                },
                NetworkNode {
                    name: "Causality clinic Toronto",
                    layer: "site",
                    focus: "Protocol coordination",
                    status: "alert",
                    coverage: 63,
                    readiness: 59,
                },
                NetworkNode {
                    name: "Patient twin lattice",
                    layer: "patient",
                    focus: "Longitudinal intervention tracking",
                    status: "nominal",
                    coverage: 88,
                    readiness: 77,
                },
                NetworkNode {
                    name: "Spatial tissue atlas",
                    layer: "tissue",
                    focus: "Microenvironment gradients",
                    status: "warming",
                    coverage: 69,
                    readiness: 72,
                },
                NetworkNode {
                    name: "Single-cell perturbation bank",
                    layer: "cell",
                    focus: "Causal mechanism inference",
                    status: "nominal",
                    coverage: 91,
                    readiness: 84,
                },
            ],
            work_queue: vec![
                WorkItem {
                    title: "Link whole-earth shocks to cohort enrollment drift",
                    layer: "whole-earth -> cohort",
                    owner: "ops mesh",
                    priority: "critical",
                    summary: "Join weather anomalies, supply constraints, and enrollment delay signals.",
                },
                WorkItem {
                    title: "Reconcile site assay lag with tissue atlas updates",
                    layer: "site -> tissue",
                    owner: "lab coordination",
                    priority: "high",
                    summary: "Detect whether biopsy backlog is distorting tissue-level trend estimates.",
                },
                WorkItem {
                    title: "Promote patient digital twin feedback loop",
                    layer: "patient",
                    owner: "care intelligence",
                    priority: "high",
                    summary: "Surface intervention effects before the next protocol steering meeting.",
                },
                WorkItem {
                    title: "Verify cell-state handoff into intervention design",
                    layer: "cell -> intervention",
                    owner: "mechanism team",
                    priority: "active",
                    summary: "Confirm single-cell signatures agree with therapeutic selection logic.",
                },
                WorkItem {
                    title: "Publish evidence digest for governance review",
                    layer: "network",
                    owner: "governance",
                    priority: "scheduled",
                    summary: "Summarize readiness, coverage, and outstanding provenance gaps.",
                },
            ],
            throughput: vec![31, 42, 37, 51, 48, 57, 63, 59, 67, 72, 69, 78],
            signal_curve: vec![
                (0.0, 41.0),
                (1.0, 43.5),
                (2.0, 47.0),
                (3.0, 49.5),
                (4.0, 54.0),
                (5.0, 57.0),
                (6.0, 60.5),
                (7.0, 59.0),
                (8.0, 63.5),
                (9.0, 67.0),
                (10.0, 69.5),
                (11.0, 73.0),
            ],
            sync_ratio: 0.74,
            evidence_ratio: 0.81,
        }
    }

    fn scale_label(&self) -> &'static str {
        SCALES[self.scale_index]
    }

    fn current_node(&self) -> &NetworkNode {
        &self.network_nodes[self.selected_network]
    }

    fn current_item(&self) -> &WorkItem {
        &self.work_queue[self.selected_queue]
    }

    fn next_scale(&mut self) {
        self.scale_index = (self.scale_index + 1) % SCALES.len();
        self.status = format!("Scale shifted to {}", self.scale_label());
    }

    fn previous_scale(&mut self) {
        self.scale_index = if self.scale_index == 0 {
            SCALES.len() - 1
        } else {
            self.scale_index - 1
        };
        self.status = format!("Scale shifted to {}", self.scale_label());
    }

    fn next_focus(&mut self) {
        self.focus = self.focus.next();
        self.status = format!("Focus moved to {}", self.focus.label());
    }

    fn select_next(&mut self) {
        match self.focus {
            FocusPane::Network => {
                self.selected_network = (self.selected_network + 1) % self.network_nodes.len();
                self.status = format!("Selected {}", self.current_node().name);
            }
            FocusPane::Queue => {
                self.selected_queue = (self.selected_queue + 1) % self.work_queue.len();
                self.status = format!("Queued {}", self.current_item().title);
            }
            FocusPane::Detail => {
                self.next_scale();
            }
        }
    }

    fn select_previous(&mut self) {
        match self.focus {
            FocusPane::Network => {
                self.selected_network = if self.selected_network == 0 {
                    self.network_nodes.len() - 1
                } else {
                    self.selected_network - 1
                };
                self.status = format!("Selected {}", self.current_node().name);
            }
            FocusPane::Queue => {
                self.selected_queue = if self.selected_queue == 0 {
                    self.work_queue.len() - 1
                } else {
                    self.selected_queue - 1
                };
                self.status = format!("Queued {}", self.current_item().title);
            }
            FocusPane::Detail => {
                self.previous_scale();
            }
        }
    }

    fn refresh(&mut self) {
        self.tick += 1;
        self.last_refresh = Instant::now();

        let lead = self.throughput.remove(0);
        let bump = 28 + ((self.tick as u64 * 7 + lead) % 57);
        self.throughput.push(bump);

        let _ = self.signal_curve.remove(0);
        let tail_x = self
            .signal_curve
            .last()
            .map(|(x, _)| x + 1.0)
            .unwrap_or(0.0);
        let tail_y = 44.0 + ((self.tick as f64 * 3.7) % 28.0);
        self.signal_curve.push((tail_x, tail_y));

        self.sync_ratio = 0.58 + (((self.tick * 3) % 37) as f64 / 100.0);
        self.evidence_ratio = 0.63 + (((self.tick * 5) % 29) as f64 / 100.0);
        self.status = format!("Refresh {} committed across {}", self.tick, self.scale_label());
    }
}

fn status_color(status: &str) -> Color {
    match status {
        "nominal" => Color::Green,
        "warming" => Color::Yellow,
        "alert" => Color::Red,
        _ => Color::Gray,
    }
}

fn gauge_color(ratio: f64) -> Color {
    if ratio >= 0.8 {
        Color::Green
    } else if ratio >= 0.65 {
        Color::Yellow
    } else {
        Color::Red
    }
}

fn focus_title(label: &str, active: bool) -> String {
    if active {
        format!("{label} *")
    } else {
        label.to_string()
    }
}

fn draw_ui(f: &mut Frame, app: &App) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(12),
            Constraint::Length(10),
            Constraint::Length(2),
        ])
        .split(f.area());

    draw_tabs(f, app, outer[0]);
    draw_summary(f, app, outer[1]);
    draw_gauges(f, app, outer[2]);
    draw_main_row(f, app, outer[3]);
    draw_analytics(f, app, outer[4]);
    draw_footer(f, app, outer[5]);
}

fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles = SCALES
        .iter()
        .map(|label| Line::from(Span::styled(*label, Style::default().fg(Color::Cyan))))
        .collect::<Vec<_>>();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" Multiscale Health Innovation Network "))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD))
        .select(app.scale_index)
        .divider(" | ");

    f.render_widget(tabs, area);
}

fn draw_summary(f: &mut Frame, app: &App, area: Rect) {
    let node = app.current_node();
    let item = app.current_item();
    let text = Line::from(vec![
        Span::styled("scale ", Style::default().fg(Color::Yellow)),
        Span::styled(app.scale_label(), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::raw("  |  "),
        Span::styled("network ", Style::default().fg(Color::Yellow)),
        Span::raw(node.name),
        Span::raw("  |  "),
        Span::styled("queue ", Style::default().fg(Color::Yellow)),
        Span::raw(item.priority),
        Span::raw(" / "),
        Span::raw(item.owner),
    ]);

    let summary = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title(" Current Slice "))
        .style(Style::default().fg(Color::Gray));

    f.render_widget(summary, area);
}

fn draw_gauges(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(34), Constraint::Percentage(33), Constraint::Percentage(33)])
        .split(area);

    let selected = app.current_node();
    let coverage_ratio = selected.coverage as f64 / 100.0;
    let readiness_ratio = selected.readiness as f64 / 100.0;

    let coverage = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Coverage "))
        .gauge_style(Style::default().fg(gauge_color(coverage_ratio)).bg(Color::DarkGray))
        .ratio(coverage_ratio)
        .label(format!("{}%", selected.coverage));

    let readiness = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Readiness "))
        .gauge_style(Style::default().fg(gauge_color(readiness_ratio)).bg(Color::DarkGray))
        .ratio(readiness_ratio)
        .label(format!("{}%", selected.readiness));

    let evidence = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Evidence Sync "))
        .gauge_style(Style::default().fg(gauge_color(app.evidence_ratio)).bg(Color::DarkGray))
        .ratio(app.evidence_ratio)
        .label(format!("{:.0}%", app.evidence_ratio * 100.0));

    f.render_widget(coverage, chunks[0]);
    f.render_widget(readiness, chunks[1]);
    f.render_widget(evidence, chunks[2]);
}

fn draw_main_row(f: &mut Frame, app: &App, area: Rect) {
    let row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(58), Constraint::Percentage(42)])
        .split(area);

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
        .split(row[0]);

    draw_network_table(f, app, left[0]);
    draw_queue(f, app, left[1]);
    draw_detail(f, app, row[1]);
}

fn draw_network_table(f: &mut Frame, app: &App, area: Rect) {
    let rows = app.network_nodes.iter().map(|node| {
        let status = Span::styled(node.status, Style::default().fg(status_color(node.status)));
        Row::new(vec![
            Cell::from(node.name),
            Cell::from(node.layer),
            Cell::from(node.focus),
            Cell::from(status),
            Cell::from(format!("{}%", node.coverage)),
            Cell::from(format!("{}%", node.readiness)),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(24),
            Constraint::Length(12),
            Constraint::Length(22),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
        ],
    )
    .header(
        Row::new(vec!["Node", "Layer", "Focus", "Status", "Coverage", "Readiness"])
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(focus_title("Network Mesh", matches!(app.focus, FocusPane::Network))),
    )
    .row_highlight_style(Style::default().bg(Color::Rgb(20, 34, 48)).add_modifier(Modifier::BOLD))
    .column_spacing(1);

    let mut state = TableState::default();
    state.select(Some(app.selected_network));
    f.render_stateful_widget(table, area, &mut state);
}

fn draw_queue(f: &mut Frame, app: &App, area: Rect) {
    let items = app
        .work_queue
        .iter()
        .map(|item| {
            let line = Line::from(vec![
                Span::styled(item.priority, Style::default().fg(Color::Yellow)),
                Span::raw("  "),
                Span::styled(item.title, Style::default().fg(Color::White)),
            ]);
            ListItem::new(vec![line, Line::from(Span::styled(item.summary, Style::default().fg(Color::Gray)))])
        })
        .collect::<Vec<_>>();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(focus_title("Work Queue", matches!(app.focus, FocusPane::Queue))),
        )
        .highlight_style(Style::default().bg(Color::Rgb(22, 32, 46)).fg(Color::White))
        .highlight_symbol(">> ");

    let mut state = ListState::default();
    state.select(Some(app.selected_queue));
    f.render_stateful_widget(list, area, &mut state);
}

fn draw_detail(f: &mut Frame, app: &App, area: Rect) {
    let node = app.current_node();
    let item = app.current_item();
    let detail = Text::from(vec![
        Line::from(vec![
            Span::styled("selected node", Style::default().fg(Color::Yellow)),
            Span::raw("  "),
            Span::styled(node.name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("layer", Style::default().fg(Color::Yellow)),
            Span::raw(format!("  {}", node.layer)),
        ]),
        Line::from(vec![
            Span::styled("focus", Style::default().fg(Color::Yellow)),
            Span::raw(format!("  {}", node.focus)),
        ]),
        Line::from(vec![
            Span::styled("status", Style::default().fg(Color::Yellow)),
            Span::raw("  "),
            Span::styled(node.status, Style::default().fg(status_color(node.status)).add_modifier(Modifier::BOLD)),
        ]),
        Line::default(),
        Line::from(vec![
            Span::styled("active queue item", Style::default().fg(Color::Yellow)),
            Span::raw("  "),
            Span::styled(item.title, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("owner", Style::default().fg(Color::Yellow)),
            Span::raw(format!("  {}", item.owner)),
        ]),
        Line::from(vec![
            Span::styled("multiscale path", Style::default().fg(Color::Yellow)),
            Span::raw(format!("  {}", item.layer)),
        ]),
        Line::default(),
        Line::from(vec![
            Span::styled("provenance", Style::default().fg(Color::Yellow)),
            Span::raw("  linked signals from care delivery, assay throughput, and intervention review"),
        ]),
        Line::from(vec![
            Span::styled("posture", Style::default().fg(Color::Yellow)),
            Span::raw("  treat the interface as an instrument for tracing causal handoffs"),
        ]),
        Line::from(vec![
            Span::styled("next action", Style::default().fg(Color::Yellow)),
            Span::raw("  confirm evidence completeness before pushing protocol changes downstream"),
        ]),
    ]);

    let detail = Paragraph::new(detail)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(focus_title("Evidence Detail", matches!(app.focus, FocusPane::Detail))),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(detail, area);
}

fn draw_analytics(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
        .split(area);

    let dataset = Dataset::default()
        .name("cross-scale signal")
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Cyan))
        .marker(symbols::Marker::Braille)
        .data(&app.signal_curve);

    let chart = Chart::new(vec![dataset])
        .block(Block::default().borders(Borders::ALL).title(" Signal Trajectory "))
        .x_axis(
            Axis::default()
                .title("cycle")
                .bounds([0.0, 12.0])
                .labels(["0", "6", "12"])
                .style(Style::default().fg(Color::Gray)),
        )
        .y_axis(
            Axis::default()
                .title("score")
                .bounds([35.0, 80.0])
                .labels(["35", "58", "80"])
                .style(Style::default().fg(Color::Gray)),
        );

    let sparkline = Sparkline::default()
        .block(Block::default().borders(Borders::ALL).title(" Assay Throughput "))
        .data(&app.throughput)
        .style(Style::default().fg(Color::Green))
        .max(100);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Min(3)])
        .split(chunks[1]);

    let sync = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Live Sync "))
        .gauge_style(Style::default().fg(gauge_color(app.sync_ratio)).bg(Color::DarkGray))
        .ratio(app.sync_ratio)
        .label(format!("{:.0}%", app.sync_ratio * 100.0));

    f.render_widget(chart, chunks[0]);
    f.render_widget(sync, right[0]);
    f.render_widget(sparkline, right[1]);
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let elapsed = app.last_refresh.elapsed().as_secs_f32();
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" q", Style::default().fg(Color::Yellow)), Span::raw(":quit "),
        Span::styled("tab", Style::default().fg(Color::Yellow)), Span::raw(":focus "),
        Span::styled("←/→", Style::default().fg(Color::Yellow)), Span::raw(":scale "),
        Span::styled("j/k", Style::default().fg(Color::Yellow)), Span::raw(":select "),
        Span::styled(" r", Style::default().fg(Color::Yellow)), Span::raw(":refresh "),
        Span::raw(format!(" | focus {} | tick {} | {:.1}s since refresh | {}",
            app.focus.label(), app.tick, elapsed, app.status)),
    ]))
    .style(Style::default().fg(Color::Gray));

    f.render_widget(footer, area);
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, mut app: App) -> Result<(), Box<dyn Error>> {
    let tick_rate = Duration::from_millis(1200);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| draw_ui(f, &app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Tab => app.next_focus(),
                        KeyCode::Left | KeyCode::Char('h') => app.previous_scale(),
                        KeyCode::Right | KeyCode::Char('l') => app.next_scale(),
                        KeyCode::Down | KeyCode::Char('j') => app.select_next(),
                        KeyCode::Up | KeyCode::Char('k') => app.select_previous(),
                        KeyCode::Char('r') => app.refresh(),
                        KeyCode::Char('1') => app.focus = FocusPane::Network,
                        KeyCode::Char('2') => app.focus = FocusPane::Queue,
                        KeyCode::Char('3') => app.focus = FocusPane::Detail,
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.refresh();
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let result = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}
