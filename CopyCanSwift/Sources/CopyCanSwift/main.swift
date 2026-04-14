import Cocoa
import Carbon

struct HistoryEntry {
    let text: String
    let timestamp: UInt64
}

class ClipboardManager {
    var entries: [HistoryEntry] = []
    let maxHistory = 50
    var lastChangeCount: Int = 0
    let pasteboard = NSPasteboard.general
    let dataFile: URL
    
    init() {
        let homeDir = URL(fileURLWithPath: ProcessInfo.processInfo.environment["HOME"] ?? ".")
        self.dataFile = homeDir.appendingPathComponent(".clipboard_history.txt")
        loadHistory()
        lastChangeCount = pasteboard.changeCount
    }
    
    func loadHistory() {
        guard let data = try? Data(contentsOf: dataFile) else { return }
        var cursor = 0
        while cursor < data.count {
            guard let tsEnd = data[cursor...].firstIndex(of: 10) else { break }
            let tsStr = String(data: data[cursor..<tsEnd], encoding: .utf8) ?? ""
            cursor = tsEnd + 1
            
            guard let lenEnd = data[cursor...].firstIndex(of: 10) else { break }
            let lenStr = String(data: data[cursor..<lenEnd], encoding: .utf8) ?? ""
            cursor = lenEnd + 1
            
            if let ts = UInt64(tsStr), let len = Int(lenStr) {
                let endIdx = min(cursor + len, data.count)
                let content = String(data: data[cursor..<endIdx], encoding: .utf8) ?? ""
                cursor = endIdx
                if cursor < data.count && data[cursor] == 10 {
                    cursor += 1
                }
                entries.append(HistoryEntry(text: content, timestamp: ts))
            } else { break }
        }
    }
    
    func saveHistory() {
        var str = ""
        for e in entries {
            let textLen = e.text.utf8.count
            str += "\(e.timestamp)\n\(textLen)\n\(e.text)\n"
        }
        try? str.data(using: .utf8)?.write(to: dataFile)
    }
    
    func checkClipboard() -> String? {
        if pasteboard.changeCount != lastChangeCount {
            lastChangeCount = pasteboard.changeCount
            if let str = pasteboard.string(forType: .string) {
                let trimmed = str.trimmingCharacters(in: .whitespacesAndNewlines)
                if !trimmed.isEmpty {
                    return str
                }
            }
        }
        return nil
    }
    
    func addEntry(_ text: String) {
        entries.removeAll { $0.text == text }
        entries.insert(HistoryEntry(text: text, timestamp: UInt64(Date().timeIntervalSince1970)), at: 0)
        if entries.count > maxHistory { entries.removeLast() }
        saveHistory()
    }
}

class AppDelegate: NSObject, NSApplicationDelegate, NSMenuDelegate {
    var statusItem: NSStatusItem!
    let manager = ClipboardManager()
    var timer: Timer?
    var hotKeyRef: EventHotKeyRef?
    var globalMenu: NSMenu?
    
    func applicationDidFinishLaunching(_ notification: Notification) {
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)
        statusItem.button?.title = "📋"
        
        rebuildMenu()
        
        timer = Timer.scheduledTimer(withTimeInterval: 0.5, repeats: true) { [weak self] _ in
            if let newText = self?.manager.checkClipboard() {
                if self?.manager.entries.first?.text != newText {
                    self?.manager.addEntry(newText)
                    self?.rebuildMenu()
                }
            }
        }
        
        registerGlobalHotkey()
    }
    
    func rebuildMenu() {
        let menu = NSMenu()
        menu.delegate = self
        menu.addItem(NSMenuItem(title: "📋 Clipboard History", action: nil, keyEquivalent: ""))
        menu.addItem(NSMenuItem.separator())
        
        if manager.entries.isEmpty {
            menu.addItem(NSMenuItem(title: "No history yet", action: nil, keyEquivalent: ""))
        } else {
            for (idx, entry) in manager.entries.enumerated() {
                let preview = String(entry.text.replacingOccurrences(of: "\n", with: " ").prefix(40)) + (entry.text.count > 40 ? "..." : "")
                let item = NSMenuItem(title: "\(idx + 1). \(preview)", action: #selector(itemClicked(_:)), keyEquivalent: "")
                item.target = self
                item.tag = idx
                menu.addItem(item)
            }
            menu.addItem(NSMenuItem.separator())
        }
        
        let clearItem = NSMenuItem(title: "Clear History", action: #selector(clearClicked), keyEquivalent: "")
        clearItem.target = self
        menu.addItem(clearItem)
        
        let quitItem = NSMenuItem(title: "Quit", action: #selector(quitClicked), keyEquivalent: "")
        quitItem.target = self
        menu.addItem(quitItem)
        
        statusItem.menu = menu
        globalMenu = menu
    }
    
    @objc func itemClicked(_ sender: NSMenuItem) {
        let entry = manager.entries[sender.tag]
        let pb = NSPasteboard.general
        pb.clearContents()
        pb.setString(entry.text, forType: .string)
        manager.lastChangeCount = pb.changeCount
        
        manager.addEntry(entry.text)
        rebuildMenu()
    }
    
    @objc func clearClicked() {
        manager.entries.removeAll()
        manager.saveHistory()
        rebuildMenu()
    }
    
    @objc func quitClicked() {
        NSApplication.shared.terminate(self)
    }
    
    func registerGlobalHotkey() {
        let modifierFlags = UInt32(cmdKey | shiftKey)
        let keyCode = UInt32(kVK_ANSI_V)
        let hotKeyId = EventHotKeyID(signature: OSType(0x434F5059), id: UInt32(1))
        
        let status = RegisterEventHotKey(keyCode, modifierFlags, hotKeyId, GetApplicationEventTarget(), 0, &hotKeyRef)
        if status != noErr {
            print("Failed to register global hotkey")
        }
        
        var eventType = EventTypeSpec(eventClass: OSType(kEventClassKeyboard), eventKind: UInt32(kEventHotKeyPressed))
        let handler: EventHandlerUPP = { (_, event, userData) -> OSStatus in
            let delegate = Unmanaged<AppDelegate>.fromOpaque(userData!).takeUnretainedValue()
            DispatchQueue.main.async {
                delegate.showMenuAtCursor()
            }
            return noErr
        }
        InstallEventHandler(GetApplicationEventTarget(), handler, 1, &eventType, Unmanaged.passUnretained(self).toOpaque(), nil)
    }
    
    func showMenuAtCursor() {
        if let menu = globalMenu {
            menu.popUp(positioning: nil, at: NSEvent.mouseLocation, in: nil)
        }
    }
}

let app = NSApplication.shared
let delegate = AppDelegate()
app.delegate = delegate
app.setActivationPolicy(.accessory)
app.run()
