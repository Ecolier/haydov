package main

import (
    "fmt"
    "os"
    "os/exec"
    "strings"

    "github.com/charmbracelet/bubbles/key"
    "github.com/charmbracelet/bubbles/list"
    "github.com/charmbracelet/bubbles/spinner"
    "github.com/charmbracelet/bubbles/viewport"
    tea "github.com/charmbracelet/bubbletea"
    "github.com/charmbracelet/lipgloss"
)

// Styles
var (
    titleStyle = lipgloss.NewStyle().
        Foreground(lipgloss.Color("#FFFDF5")).
        Background(lipgloss.Color("#25A065")).
        Padding(0, 1)

    statusRunning = lipgloss.NewStyle().Foreground(lipgloss.Color("#04B575"))
    statusStopped = lipgloss.NewStyle().Foreground(lipgloss.Color("#EF4444"))
    statusUnknown = lipgloss.NewStyle().Foreground(lipgloss.Color("#F59E0B"))

    focusedStyle = lipgloss.NewStyle().Foreground(lipgloss.Color("#01FAC6")).Bold(true)
    blurredStyle = lipgloss.NewStyle().Foreground(lipgloss.Color("#626262"))
)

// Service represents a development service
type Service struct {
    Name        string
    Dir         string
    Type        string // "infrastructure", "backend", "frontend", "mobile"
    Status      string // "running", "stopped", "unknown"
    Port        string
    Description string
}

// Command represents an available development command
type Command struct {
    Name        string
    Description string
    Category    string
    Action      func() tea.Cmd
}

// Model represents the TUI state
type Model struct {
    services    []Service
    commands    []Command
    activeTab   int
    serviceList list.Model
    commandList list.Model
    logs        viewport.Model
    spinner     spinner.Model
    loading     bool
    width       int
    height      int
}

// Tab names
const (
    servicesTab = 0
    commandsTab = 1
    logsTab     = 2
)

var tabs = []string{"Services", "Commands", "Logs"}

// Initialize services
func initServices() []Service {
    return []Service{
        {
            Name:        "message-broker",
            Dir:         "common/services/message-broker",
            Type:        "infrastructure",
            Status:      "stopped",
            Port:        "5672",
            Description: "RabbitMQ message broker",
        },
        {
            Name:        "maps-storage",
            Dir:         "maps/services/storage",
            Type:        "infrastructure", 
            Status:      "stopped",
            Port:        "5432",
            Description: "PostgreSQL storage for maps",
        },
        {
            Name:        "geography-dispatcher",
            Dir:         "services/geography/dispatcher",
            Type:        "backend",
            Status:      "stopped",
            Port:        "8001",
            Description: "Rust routing service",
        },
        {
            Name:        "geography-importer",
            Dir:         "services/geography/importer",
            Type:        "backend",
            Status:      "stopped",
            Port:        "8002",
            Description: "Node.js data processing",
        },
        {
            Name:        "mobile-app",
            Dir:         "apps/mobile",
            Type:        "mobile",
            Status:      "stopped",
            Port:        "8081",
            Description: "Expo React Native app",
        },
    }
}

// Initialize commands
func initCommands(m *Model) []Command {
    return []Command{
        {
            Name:        "Start All Services",
            Description: "Start all development services",
            Category:    "Services",
            Action:      func() tea.Cmd { return startAllServices },
        },
        {
            Name:        "Stop All Services", 
            Description: "Stop all running services",
            Category:    "Services",
            Action:      func() tea.Cmd { return stopAllServices },
        },
        {
            Name:        "Build Docker Images",
            Description: "Build all Docker images for services",
            Category:    "Build",
            Action:      func() tea.Cmd { return buildImages },
        },
        {
            Name:        "Run Tests",
            Description: "Run all tests (pnpm + cargo)",
            Category:    "Testing",
            Action:      func() tea.Cmd { return runTests },
        },
        {
            Name:        "Start Tilt",
            Description: "Start Tilt development environment",
            Category:    "Development",
            Action:      func() tea.Cmd { return startTilt },
        },
        {
            Name:        "Mobile Dev Server",
            Description: "Start Expo development server",
            Category:    "Mobile",
            Action:      func() tea.Cmd { return startMobile },
        },
        {
            Name:        "Clean All",
            Description: "Clean all services and build artifacts",
            Category:    "Maintenance",
            Action:      func() tea.Cmd { return cleanAll },
        },
    }
}

// Messages
type serviceStatusMsg struct {
    services []Service
}

type commandCompleteMsg struct {
    output string
    err    error
}

type logMsg string

// Commands
func checkServiceStatus() tea.Cmd {
    return func() tea.Msg {
        services := initServices()
        for i := range services {
            services[i].Status = getServiceStatus(services[i].Name)
        }
        return serviceStatusMsg{services}
    }
}

func getServiceStatus(serviceName string) string {
    var cmd *exec.Cmd
    switch serviceName {
    case "message-broker":
        cmd = exec.Command("pgrep", "-f", "rabbitmq")
    case "maps-storage":
        cmd = exec.Command("pgrep", "-f", "postgres.*haydov")
    case "geography-dispatcher":
        cmd = exec.Command("pgrep", "-f", "geography.*dispatcher")
    case "geography-importer":
        cmd = exec.Command("pgrep", "-f", "geography.*importer")
    case "mobile-app":
        cmd = exec.Command("pgrep", "-f", "expo.*start")
    default:
        return "unknown"
    }

    if err := cmd.Run(); err != nil {
        return "stopped"
    }
    return "running"
}

func startAllServices() tea.Msg {
    cmd := exec.Command("./scripts/services.sh", "start")
    output, err := cmd.CombinedOutput()
    return commandCompleteMsg{string(output), err}
}

func stopAllServices() tea.Msg {
    cmd := exec.Command("./scripts/services.sh", "stop")
    output, err := cmd.CombinedOutput()
    return commandCompleteMsg{string(output), err}
}

func buildImages() tea.Msg {
    cmd := exec.Command("nix", "run", ".#build-images")
    output, err := cmd.CombinedOutput()
    return commandCompleteMsg{string(output), err}
}

func runTests() tea.Msg {
    cmd := exec.Command("bash", "-c", "pnpm nx test && cargo test --workspace")
    output, err := cmd.CombinedOutput()
    return commandCompleteMsg{string(output), err}
}

func startTilt() tea.Msg {
    cmd := exec.Command("tilt", "up")
    output, err := cmd.CombinedOutput()
    return commandCompleteMsg{string(output), err}
}

func startMobile() tea.Msg {
    cmd := exec.Command("bash", "-c", "cd apps/mobile && npx expo start")
    output, err := cmd.CombinedOutput()
    return commandCompleteMsg{string(output), err}
}

func cleanAll() tea.Msg {
    cmd := exec.Command("./scripts/services.sh", "clean")
    output, err := cmd.CombinedOutput()
    return commandCompleteMsg{string(output), err}
}

// List item implementations
type serviceItem struct {
    service Service
}

func (s serviceItem) Title() string { return s.service.Name }
func (s serviceItem) Description() string { 
    status := s.service.Status
    var statusStr string
    switch status {
    case "running":
        statusStr = statusRunning.Render("● running")
    case "stopped":
        statusStr = statusStopped.Render("● stopped")
    default:
        statusStr = statusUnknown.Render("● unknown")
    }
    return fmt.Sprintf("%s - %s | Port: %s", statusStr, s.service.Description, s.service.Port)
}
func (s serviceItem) FilterValue() string { return s.service.Name }

type commandItem struct {
    command Command
}

func (c commandItem) Title() string { return c.command.Name }
func (c commandItem) Description() string { 
    return fmt.Sprintf("[%s] %s", c.command.Category, c.command.Description)
}
func (c commandItem) FilterValue() string { return c.command.Name }

// Key bindings
type keyMap struct {
    Tab    key.Binding
    Enter  key.Binding
    Quit   key.Binding
    Help   key.Binding
    Refresh key.Binding
}

var keys = keyMap{
    Tab: key.NewBinding(
        key.WithKeys("tab"),
        key.WithHelp("tab", "switch tabs"),
    ),
    Enter: key.NewBinding(
        key.WithKeys("enter"),
        key.WithHelp("enter", "execute"),
    ),
    Quit: key.NewBinding(
        key.WithKeys("q", "ctrl+c"),
        key.WithHelp("q", "quit"),
    ),
    Help: key.NewBinding(
        key.WithKeys("?"),
        key.WithHelp("?", "help"),
    ),
    Refresh: key.NewBinding(
        key.WithKeys("r"),
        key.WithHelp("r", "refresh"),
    ),
}

func initialModel() Model {
    // Initialize services
    services := initServices()
    
    // Create service list
    serviceItems := make([]list.Item, len(services))
    for i, service := range services {
        serviceItems[i] = serviceItem{service}
    }
    serviceList := list.New(serviceItems, list.NewDefaultDelegate(), 0, 0)
    serviceList.Title = "Development Services"

    // Initialize model
    m := Model{
        services: services,
        serviceList: serviceList,
        spinner:  spinner.New(),
        logs:     viewport.New(0, 0),
    }

    // Initialize commands
    commands := initCommands(&m)
    commandItems := make([]list.Item, len(commands))
    for i, command := range commands {
        commandItems[i] = commandItem{command}
    }
    commandList := list.New(commandItems, list.NewDefaultDelegate(), 0, 0)
    commandList.Title = "Development Commands"
    m.commandList = commandList
    m.commands = commands

    m.spinner.Spinner = spinner.Dot
    m.logs.SetContent("Welcome to Haydov Development Environment\n\nPress 'r' to refresh service status\nPress 'tab' to switch between tabs\nPress 'enter' to execute commands\n")

    return m
}

func (m Model) Init() tea.Cmd {
    return tea.Batch(
        checkServiceStatus(),
        m.spinner.Tick,
    )
}

func (m Model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
    var cmds []tea.Cmd

    switch msg := msg.(type) {
    case tea.KeyMsg:
        switch {
        case key.Matches(msg, keys.Quit):
            return m, tea.Quit
        case key.Matches(msg, keys.Tab):
            m.activeTab = (m.activeTab + 1) % len(tabs)
            return m, nil
        case key.Matches(msg, keys.Refresh):
            return m, checkServiceStatus()
        case key.Matches(msg, keys.Enter):
            if m.activeTab == commandsTab {
                selected := m.commandList.SelectedItem()
                if cmd, ok := selected.(commandItem); ok {
                    m.loading = true
                    return m, tea.Batch(cmd.command.Action(), m.spinner.Tick)
                }
            }
        }

    case tea.WindowSizeMsg:
        m.width = msg.Width
        m.height = msg.Height
        
        listHeight := m.height - 5
        m.serviceList.SetSize(m.width, listHeight)
        m.commandList.SetSize(m.width, listHeight)
        m.logs.Width = m.width
        m.logs.Height = listHeight

    case serviceStatusMsg:
        m.services = msg.services
        items := make([]list.Item, len(m.services))
        for i, service := range m.services {
            items[i] = serviceItem{service}
        }
        return m, m.serviceList.SetItems(items)

    case commandCompleteMsg:
        m.loading = false
        logContent := m.logs.View()
        if msg.err != nil {
            logContent += fmt.Sprintf("\n❌ Error: %s\n%s\n", msg.err.Error(), msg.output)
        } else {
            logContent += fmt.Sprintf("\n✅ Command completed successfully\n%s\n", msg.output)
        }
        m.logs.SetContent(logContent)
        m.logs.GotoBottom()
        return m, checkServiceStatus()

    case spinner.TickMsg:
        var cmd tea.Cmd
        m.spinner, cmd = m.spinner.Update(msg)
        return m, cmd
    }

    // Update active tab content
    var cmd tea.Cmd
    switch m.activeTab {
    case servicesTab:
        m.serviceList, cmd = m.serviceList.Update(msg)
    case commandsTab:
        m.commandList, cmd = m.commandList.Update(msg)
    case logsTab:
        m.logs, cmd = m.logs.Update(msg)
    }
    cmds = append(cmds, cmd)

    return m, tea.Batch(cmds...)
}

func (m Model) View() string {
    if m.width == 0 {
        return "Loading..."
    }

    // Header
    header := titleStyle.Render("🌎 Haydov Development Environment")
    
    // Tab bar
    var tabBar []string
    for i, tab := range tabs {
        if i == m.activeTab {
            tabBar = append(tabBar, focusedStyle.Render(fmt.Sprintf("[%s]", tab)))
        } else {
            tabBar = append(tabBar, blurredStyle.Render(tab))
        }
    }
    
    // Content based on active tab
    var content string
    switch m.activeTab {
    case servicesTab:
        content = m.serviceList.View()
    case commandsTab:
        if m.loading {
            content = fmt.Sprintf("%s Executing command...\n\n%s", 
                m.spinner.View(), m.commandList.View())
        } else {
            content = m.commandList.View()
        }
    case logsTab:
        content = m.logs.View()
    }

    // Footer
    footer := blurredStyle.Render("tab: switch • enter: execute • r: refresh • q: quit")

    return fmt.Sprintf("%s\n%s\n\n%s\n\n%s",
        header,
        strings.Join(tabBar, " "),
        content,
        footer,
    )
}

func main() {
    p := tea.NewProgram(initialModel(), tea.WithAltScreen())
    if _, err := p.Run(); err != nil {
        fmt.Printf("Error running program: %v", err)
        os.Exit(1)
    }
}