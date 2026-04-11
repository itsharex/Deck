use std::collections::HashMap;
use std::sync::LazyLock;

type Map = HashMap<&'static str, &'static str>;
type Locales = HashMap<&'static str, Map>;

static LOCALES: LazyLock<Locales> = LazyLock::new(|| {
    let mut m = Locales::new();
    m.insert("zh-Hans", zh_hans());
    m.insert("en", en());
    m.insert("de", de());
    m.insert("fr", fr());
    m.insert("ja", ja());
    m.insert("ko", ko());
    m.insert("zh-Hant", zh_hant());
    m
});

pub fn get(locale: &str, key: &str) -> Option<String> {
    LOCALES
        .get(locale)
        .and_then(|m| m.get(key))
        .map(|s| s.to_string())
}

// ─── zh-Hans (source) ───

fn zh_hans() -> Map {
    HashMap::from([
        // CLI top-level
        ("cli.about", "DeckClip — Deck 剪贴板管理工具的命令行接口"),
        ("cli.long_about", "DeckClip — Deck 剪贴板管理工具的命令行接口\n\nAI Agent 可直接调用上述命令操作 Deck 剪贴板。\n详细用法: deckclip <command> --help"),
        ("arg.json", "所有输出使用 JSON 格式（适用于编程调用）"),

        // Subcommands
        ("cmd.health", "检查 Deck App 连接状态"),
        ("cmd.write", "写入文本到 Deck 剪贴板"),
        ("cmd.read", "读取最新剪贴板项"),
        ("cmd.paste", "快速粘贴面板项（1-9）"),
        ("cmd.panel", "控制面板显示"),
        ("cmd.ai", "AI 功能（运行/搜索/转换）"),
        ("cmd.completion", "生成 shell 补全脚本"),
        ("cmd.version", "显示版本信息"),

        // Panel subcommands
        ("cmd.panel.toggle", "切换面板显示/隐藏"),
        ("arg.panel.action", "面板操作"),

        // AI subcommands
        ("cmd.ai.run", "运行 AI 处理"),
        ("cmd.ai.search", "AI 搜索剪贴板历史"),
        ("cmd.ai.transform", "AI 文本转换"),

        // Write args
        ("arg.write.text", "要写入的文本（省略则从 stdin 读取）"),
        ("arg.write.tag", "指定标签名"),
        ("arg.write.tag_id", "指定标签 ID"),
        ("arg.write.raw", "跳过智能规则"),

        // Paste args
        ("arg.paste.index", "面板项索引（1-9）"),
        ("arg.paste.plain", "纯文本粘贴"),
        ("arg.paste.target", "指定目标 App（Bundle ID）"),

        // AI args
        ("arg.ai.prompt", "AI 指令（prompt）"),
        ("arg.ai.text", "输入文本（省略则从 stdin 读取）"),
        ("arg.ai.save", "自动保存结果到剪贴板"),
        ("arg.ai.tag_id", "保存到指定标签 ID"),
        ("arg.ai.query", "搜索关键词"),
        ("arg.ai.mode", "搜索模式"),
        ("arg.ai.limit", "结果数量（默认 10，最大 50）"),
        ("arg.ai.transform_text", "待转换文本（省略则从 stdin 读取）"),
        ("arg.ai.plugin", "使用指定插件 ID"),
        ("arg.completion.shell", "Shell 类型"),

        // Command output
        ("health.ok", "ok — Deck App 连接正常"),
        ("write.ok", "已写入剪贴板"),
        ("paste.ok", "已粘贴第 {} 项"),
        ("panel.toggled", "面板已切换"),

        // Errors
        ("err.not_running", "Deck App 未运行或未启用 Deck CLI"),
        ("err.connection", "连接失败: {}"),
        ("err.auth", "认证失败: {}"),
        ("err.token_not_found", "Token 文件不存在: {}"),
        ("err.timeout", "请求超时"),
        ("err.protocol", "协议错误: {}"),
        ("err.server", "服务端错误 [{}]: {}"),
        ("err.io", "IO 错误: {}"),
        ("err.token_read", "无法读取 token 文件 {}: {}"),
        ("err.token_empty", "token 文件为空"),
        ("err.conn_closed", "连接已关闭"),
        ("err.auth_rejected", "认证被拒绝"),
        ("err.no_session", "无 session token"),
        ("err.id_mismatch", "响应 ID 不匹配: expected {}, got {}"),
        ("err.stdin_hint", "请提供文本参数，或通过管道传入（如: echo \"text\" | deckclip write）"),
    ])
}

// ─── English ───

fn en() -> Map {
    HashMap::from([
        ("cli.about", "DeckClip — command-line interface for Deck clipboard manager"),
        ("cli.long_about", "DeckClip — command-line interface for Deck clipboard manager\n\nAI agents can call these commands directly to operate the Deck clipboard.\nUsage: deckclip <command> --help"),
        ("arg.json", "Output in JSON format (for programmatic use)"),

        ("cmd.health", "Check Deck App connection status"),
        ("cmd.write", "Write text to Deck clipboard"),
        ("cmd.read", "Read latest clipboard entry"),
        ("cmd.paste", "Quick-paste a panel item (1-9)"),
        ("cmd.panel", "Control the panel"),
        ("cmd.ai", "AI features (run/search/transform)"),
        ("cmd.completion", "Generate shell completion script"),
        ("cmd.version", "Show version info"),

        ("cmd.panel.toggle", "Toggle panel visibility"),
        ("arg.panel.action", "Panel action"),

        ("cmd.ai.run", "Run AI processing"),
        ("cmd.ai.search", "AI search clipboard history"),
        ("cmd.ai.transform", "AI text transformation"),

        ("arg.write.text", "Text to write (reads from stdin if omitted)"),
        ("arg.write.tag", "Tag name"),
        ("arg.write.tag_id", "Tag ID"),
        ("arg.write.raw", "Skip smart rules"),

        ("arg.paste.index", "Panel item index (1-9)"),
        ("arg.paste.plain", "Paste as plain text"),
        ("arg.paste.target", "Target app (Bundle ID)"),

        ("arg.ai.prompt", "AI prompt"),
        ("arg.ai.text", "Input text (reads from stdin if omitted)"),
        ("arg.ai.save", "Auto-save result to clipboard"),
        ("arg.ai.tag_id", "Save to tag ID"),
        ("arg.ai.query", "Search query"),
        ("arg.ai.mode", "Search mode"),
        ("arg.ai.limit", "Result count (default 10, max 50)"),
        ("arg.ai.transform_text", "Text to transform (reads from stdin if omitted)"),
        ("arg.ai.plugin", "Plugin ID"),
        ("arg.completion.shell", "Shell type"),

        ("health.ok", "ok — Deck App connected"),
        ("write.ok", "Written to clipboard"),
        ("paste.ok", "Pasted item {}"),
        ("panel.toggled", "Panel toggled"),

        ("err.not_running", "Deck App is not running or Deck CLI is disabled"),
        ("err.connection", "Connection failed: {}"),
        ("err.auth", "Authentication failed: {}"),
        ("err.token_not_found", "Token file not found: {}"),
        ("err.timeout", "Request timed out"),
        ("err.protocol", "Protocol error: {}"),
        ("err.server", "Server error [{}]: {}"),
        ("err.io", "IO error: {}"),
        ("err.token_read", "Cannot read token file {}: {}"),
        ("err.token_empty", "Token file is empty"),
        ("err.conn_closed", "Connection closed"),
        ("err.auth_rejected", "Authentication rejected"),
        ("err.no_session", "No session token"),
        ("err.id_mismatch", "Response ID mismatch: expected {}, got {}"),
        ("err.stdin_hint", "Provide text as an argument or pipe it in (e.g.: echo \"text\" | deckclip write)"),
    ])
}

// ─── German ───

fn de() -> Map {
    HashMap::from([
        ("cli.about", "DeckClip — Befehlszeilenschnittstelle für den Deck-Zwischenablage-Manager"),
        ("cli.long_about", "DeckClip — Befehlszeilenschnittstelle für den Deck-Zwischenablage-Manager\n\nKI-Agenten können diese Befehle direkt aufrufen.\nVerwendung: deckclip <command> --help"),
        ("arg.json", "Alle Ausgaben im JSON-Format (für programmatische Nutzung)"),

        ("cmd.health", "Deck App Verbindungsstatus prüfen"),
        ("cmd.write", "Text in die Deck-Zwischenablage schreiben"),
        ("cmd.read", "Letzten Eintrag lesen"),
        ("cmd.paste", "Panel-Eintrag schnell einfügen (1-9)"),
        ("cmd.panel", "Panel steuern"),
        ("cmd.ai", "KI-Funktionen (Ausführen/Suchen/Umwandeln)"),
        ("cmd.completion", "Shell-Vervollständigungsskript generieren"),
        ("cmd.version", "Versionsinformation anzeigen"),

        ("cmd.panel.toggle", "Panel ein-/ausblenden"),
        ("arg.panel.action", "Panel-Aktion"),

        ("cmd.ai.run", "KI-Verarbeitung ausführen"),
        ("cmd.ai.search", "KI-Suche im Verlauf"),
        ("cmd.ai.transform", "KI-Textumwandlung"),

        ("arg.write.text", "Zu schreibender Text (liest von stdin wenn weggelassen)"),
        ("arg.write.tag", "Tag-Name"),
        ("arg.write.tag_id", "Tag-ID"),
        ("arg.write.raw", "Intelligente Regeln überspringen"),

        ("arg.paste.index", "Panel-Eintrag-Index (1-9)"),
        ("arg.paste.plain", "Als reinen Text einfügen"),
        ("arg.paste.target", "Ziel-App (Bundle ID)"),

        ("arg.ai.prompt", "KI-Anweisung (Prompt)"),
        ("arg.ai.text", "Eingabetext (liest von stdin wenn weggelassen)"),
        ("arg.ai.save", "Ergebnis automatisch speichern"),
        ("arg.ai.tag_id", "In Tag-ID speichern"),
        ("arg.ai.query", "Suchbegriff"),
        ("arg.ai.mode", "Suchmodus"),
        ("arg.ai.limit", "Ergebnisanzahl (Standard 10, max 50)"),
        ("arg.ai.transform_text", "Umzuwandelnder Text (liest von stdin wenn weggelassen)"),
        ("arg.ai.plugin", "Plugin-ID"),
        ("arg.completion.shell", "Shell-Typ"),

        ("health.ok", "ok — Deck App verbunden"),
        ("write.ok", "In Zwischenablage geschrieben"),
        ("paste.ok", "Eintrag {} eingefügt"),
        ("panel.toggled", "Panel umgeschaltet"),

        ("err.not_running", "Deck App läuft nicht oder Deck CLI ist deaktiviert"),
        ("err.connection", "Verbindung fehlgeschlagen: {}"),
        ("err.auth", "Authentifizierung fehlgeschlagen: {}"),
        ("err.token_not_found", "Token-Datei nicht gefunden: {}"),
        ("err.timeout", "Zeitüberschreitung"),
        ("err.protocol", "Protokollfehler: {}"),
        ("err.server", "Serverfehler [{}]: {}"),
        ("err.io", "IO-Fehler: {}"),
        ("err.token_read", "Token-Datei {} kann nicht gelesen werden: {}"),
        ("err.token_empty", "Token-Datei ist leer"),
        ("err.conn_closed", "Verbindung geschlossen"),
        ("err.auth_rejected", "Authentifizierung abgelehnt"),
        ("err.no_session", "Kein Session-Token"),
        ("err.id_mismatch", "Antwort-ID stimmt nicht überein: erwartet {}, erhalten {}"),
        ("err.stdin_hint", "Bitte Text als Argument angeben oder per Pipe übergeben (z.B.: echo \"text\" | deckclip write)"),
    ])
}

// ─── French ───

fn fr() -> Map {
    HashMap::from([
        ("cli.about", "DeckClip — interface en ligne de commande pour le gestionnaire de presse-papiers Deck"),
        ("cli.long_about", "DeckClip — interface en ligne de commande pour le gestionnaire de presse-papiers Deck\n\nLes agents IA peuvent appeler ces commandes directement.\nUtilisation : deckclip <command> --help"),
        ("arg.json", "Sortie au format JSON (pour utilisation programmatique)"),

        ("cmd.health", "Vérifier la connexion à Deck App"),
        ("cmd.write", "Écrire du texte dans le presse-papiers Deck"),
        ("cmd.read", "Lire la dernière entrée"),
        ("cmd.paste", "Collage rapide d'un élément du panneau (1-9)"),
        ("cmd.panel", "Contrôler le panneau"),
        ("cmd.ai", "Fonctions IA (exécuter/rechercher/transformer)"),
        ("cmd.completion", "Générer le script de complétion shell"),
        ("cmd.version", "Afficher la version"),

        ("cmd.panel.toggle", "Afficher/masquer le panneau"),
        ("arg.panel.action", "Action du panneau"),

        ("cmd.ai.run", "Exécuter le traitement IA"),
        ("cmd.ai.search", "Recherche IA dans l'historique"),
        ("cmd.ai.transform", "Transformation de texte par IA"),

        ("arg.write.text", "Texte à écrire (lit depuis stdin si omis)"),
        ("arg.write.tag", "Nom du tag"),
        ("arg.write.tag_id", "ID du tag"),
        ("arg.write.raw", "Ignorer les règles intelligentes"),

        ("arg.paste.index", "Index de l'élément du panneau (1-9)"),
        ("arg.paste.plain", "Coller en texte brut"),
        ("arg.paste.target", "Application cible (Bundle ID)"),

        ("arg.ai.prompt", "Instruction IA (prompt)"),
        ("arg.ai.text", "Texte d'entrée (lit depuis stdin si omis)"),
        ("arg.ai.save", "Sauvegarder automatiquement le résultat"),
        ("arg.ai.tag_id", "Sauvegarder dans le tag ID"),
        ("arg.ai.query", "Mot-clé de recherche"),
        ("arg.ai.mode", "Mode de recherche"),
        ("arg.ai.limit", "Nombre de résultats (défaut 10, max 50)"),
        ("arg.ai.transform_text", "Texte à transformer (lit depuis stdin si omis)"),
        ("arg.ai.plugin", "ID du plugin"),
        ("arg.completion.shell", "Type de shell"),

        ("health.ok", "ok — Deck App connecté"),
        ("write.ok", "Écrit dans le presse-papiers"),
        ("paste.ok", "Élément {} collé"),
        ("panel.toggled", "Panneau basculé"),

        ("err.not_running", "Deck App n'est pas en cours d'exécution ou Deck CLI est désactivé"),
        ("err.connection", "Échec de connexion : {}"),
        ("err.auth", "Échec d'authentification : {}"),
        ("err.token_not_found", "Fichier token introuvable : {}"),
        ("err.timeout", "Délai d'attente dépassé"),
        ("err.protocol", "Erreur de protocole : {}"),
        ("err.server", "Erreur serveur [{}] : {}"),
        ("err.io", "Erreur IO : {}"),
        ("err.token_read", "Impossible de lire le fichier token {} : {}"),
        ("err.token_empty", "Le fichier token est vide"),
        ("err.conn_closed", "Connexion fermée"),
        ("err.auth_rejected", "Authentification rejetée"),
        ("err.no_session", "Pas de token de session"),
        ("err.id_mismatch", "ID de réponse non concordant : attendu {}, reçu {}"),
        ("err.stdin_hint", "Fournissez le texte en argument ou par pipe (ex : echo \"text\" | deckclip write)"),
    ])
}

// ─── Japanese ───

fn ja() -> Map {
    HashMap::from([
        ("cli.about", "DeckClip — Deck クリップボードマネージャーのコマンドラインインターフェース"),
        ("cli.long_about", "DeckClip — Deck クリップボードマネージャーのコマンドラインインターフェース\n\nAI エージェントはこれらのコマンドを直接呼び出すことができます。\n使い方: deckclip <command> --help"),
        ("arg.json", "すべての出力を JSON 形式で表示（プログラム利用向け）"),

        ("cmd.health", "Deck App の接続状態を確認"),
        ("cmd.write", "Deck クリップボードにテキストを書き込む"),
        ("cmd.read", "最新のクリップボード項目を読み取る"),
        ("cmd.paste", "パネル項目をクイックペースト（1-9）"),
        ("cmd.panel", "パネルの表示を制御"),
        ("cmd.ai", "AI 機能（実行/検索/変換）"),
        ("cmd.completion", "シェル補完スクリプトを生成"),
        ("cmd.version", "バージョン情報を表示"),

        ("cmd.panel.toggle", "パネルの表示/非表示を切り替え"),
        ("arg.panel.action", "パネル操作"),

        ("cmd.ai.run", "AI 処理を実行"),
        ("cmd.ai.search", "AI でクリップボード履歴を検索"),
        ("cmd.ai.transform", "AI テキスト変換"),

        ("arg.write.text", "書き込むテキスト（省略時は stdin から読み取り）"),
        ("arg.write.tag", "タグ名を指定"),
        ("arg.write.tag_id", "タグ ID を指定"),
        ("arg.write.raw", "スマートルールをスキップ"),

        ("arg.paste.index", "パネル項目のインデックス（1-9）"),
        ("arg.paste.plain", "プレーンテキストで貼り付け"),
        ("arg.paste.target", "ターゲットアプリ（Bundle ID）"),

        ("arg.ai.prompt", "AI 指示（プロンプト）"),
        ("arg.ai.text", "入力テキスト（省略時は stdin から読み取り）"),
        ("arg.ai.save", "結果を自動でクリップボードに保存"),
        ("arg.ai.tag_id", "指定タグ ID に保存"),
        ("arg.ai.query", "検索キーワード"),
        ("arg.ai.mode", "検索モード"),
        ("arg.ai.limit", "結果数（デフォルト 10、最大 50）"),
        ("arg.ai.transform_text", "変換するテキスト（省略時は stdin から読み取り）"),
        ("arg.ai.plugin", "プラグイン ID を指定"),
        ("arg.completion.shell", "シェルの種類"),

        ("health.ok", "ok — Deck App 接続正常"),
        ("write.ok", "クリップボードに書き込みました"),
        ("paste.ok", "項目 {} を貼り付けました"),
        ("panel.toggled", "パネルを切り替えました"),

        ("err.not_running", "Deck App が起動していないか、Deck CLI が無効です"),
        ("err.connection", "接続に失敗: {}"),
        ("err.auth", "認証に失敗: {}"),
        ("err.token_not_found", "トークンファイルが見つかりません: {}"),
        ("err.timeout", "リクエストがタイムアウトしました"),
        ("err.protocol", "プロトコルエラー: {}"),
        ("err.server", "サーバーエラー [{}]: {}"),
        ("err.io", "IO エラー: {}"),
        ("err.token_read", "トークンファイル {} を読み取れません: {}"),
        ("err.token_empty", "トークンファイルが空です"),
        ("err.conn_closed", "接続が閉じられました"),
        ("err.auth_rejected", "認証が拒否されました"),
        ("err.no_session", "セッショントークンがありません"),
        ("err.id_mismatch", "レスポンス ID 不一致: 期待 {}、取得 {}"),
        ("err.stdin_hint", "テキストを引数で指定するか、パイプで入力してください（例: echo \"text\" | deckclip write）"),
    ])
}

// ─── Korean ───

fn ko() -> Map {
    HashMap::from([
        ("cli.about", "DeckClip — Deck 클립보드 관리자 명령줄 인터페이스"),
        ("cli.long_about", "DeckClip — Deck 클립보드 관리자 명령줄 인터페이스\n\nAI 에이전트가 이 명령어를 직접 호출할 수 있습니다.\n사용법: deckclip <command> --help"),
        ("arg.json", "모든 출력을 JSON 형식으로 표시 (프로그래밍 사용)"),

        ("cmd.health", "Deck App 연결 상태 확인"),
        ("cmd.write", "Deck 클립보드에 텍스트 쓰기"),
        ("cmd.read", "최신 클립보드 항목 읽기"),
        ("cmd.paste", "패널 항목 빠른 붙여넣기 (1-9)"),
        ("cmd.panel", "패널 표시 제어"),
        ("cmd.ai", "AI 기능 (실행/검색/변환)"),
        ("cmd.completion", "셸 자동완성 스크립트 생성"),
        ("cmd.version", "버전 정보 표시"),

        ("cmd.panel.toggle", "패널 표시/숨기기 전환"),
        ("arg.panel.action", "패널 동작"),

        ("cmd.ai.run", "AI 처리 실행"),
        ("cmd.ai.search", "AI 클립보드 기록 검색"),
        ("cmd.ai.transform", "AI 텍스트 변환"),

        ("arg.write.text", "쓸 텍스트 (생략 시 stdin에서 읽기)"),
        ("arg.write.tag", "태그 이름"),
        ("arg.write.tag_id", "태그 ID"),
        ("arg.write.raw", "스마트 규칙 건너뛰기"),

        ("arg.paste.index", "패널 항목 인덱스 (1-9)"),
        ("arg.paste.plain", "일반 텍스트로 붙여넣기"),
        ("arg.paste.target", "대상 앱 (Bundle ID)"),

        ("arg.ai.prompt", "AI 지시 (프롬프트)"),
        ("arg.ai.text", "입력 텍스트 (생략 시 stdin에서 읽기)"),
        ("arg.ai.save", "결과를 클립보드에 자동 저장"),
        ("arg.ai.tag_id", "태그 ID에 저장"),
        ("arg.ai.query", "검색 키워드"),
        ("arg.ai.mode", "검색 모드"),
        ("arg.ai.limit", "결과 수 (기본 10, 최대 50)"),
        ("arg.ai.transform_text", "변환할 텍스트 (생략 시 stdin에서 읽기)"),
        ("arg.ai.plugin", "플러그인 ID"),
        ("arg.completion.shell", "셸 유형"),

        ("health.ok", "ok — Deck App 연결됨"),
        ("write.ok", "클립보드에 기록됨"),
        ("paste.ok", "항목 {} 붙여넣기 완료"),
        ("panel.toggled", "패널 전환됨"),

        ("err.not_running", "Deck App이 실행 중이 아니거나 Deck CLI가 비활성화되어 있습니다"),
        ("err.connection", "연결 실패: {}"),
        ("err.auth", "인증 실패: {}"),
        ("err.token_not_found", "토큰 파일을 찾을 수 없습니다: {}"),
        ("err.timeout", "요청 시간 초과"),
        ("err.protocol", "프로토콜 오류: {}"),
        ("err.server", "서버 오류 [{}]: {}"),
        ("err.io", "IO 오류: {}"),
        ("err.token_read", "토큰 파일 {}을 읽을 수 없습니다: {}"),
        ("err.token_empty", "토큰 파일이 비어 있습니다"),
        ("err.conn_closed", "연결이 닫혔습니다"),
        ("err.auth_rejected", "인증이 거부되었습니다"),
        ("err.no_session", "세션 토큰이 없습니다"),
        ("err.id_mismatch", "응답 ID 불일치: 예상 {}, 수신 {}"),
        ("err.stdin_hint", "텍스트를 인수로 제공하거나 파이프로 입력하세요 (예: echo \"text\" | deckclip write)"),
    ])
}

// ─── zh-Hant ───

fn zh_hant() -> Map {
    HashMap::from([
        ("cli.about", "DeckClip — Deck 剪貼簿管理工具的命令列介面"),
        ("cli.long_about", "DeckClip — Deck 剪貼簿管理工具的命令列介面\n\nAI Agent 可直接呼叫上述指令操作 Deck 剪貼簿。\n詳細用法: deckclip <command> --help"),
        ("arg.json", "所有輸出使用 JSON 格式（適用於程式呼叫）"),

        ("cmd.health", "檢查 Deck App 連線狀態"),
        ("cmd.write", "寫入文字到 Deck 剪貼簿"),
        ("cmd.read", "讀取最新剪貼簿項目"),
        ("cmd.paste", "快速貼上面板項目（1-9）"),
        ("cmd.panel", "控制面板顯示"),
        ("cmd.ai", "AI 功能（執行/搜尋/轉換）"),
        ("cmd.completion", "產生 shell 自動完成腳本"),
        ("cmd.version", "顯示版本資訊"),

        ("cmd.panel.toggle", "切換面板顯示/隱藏"),
        ("arg.panel.action", "面板操作"),

        ("cmd.ai.run", "執行 AI 處理"),
        ("cmd.ai.search", "AI 搜尋剪貼簿歷史"),
        ("cmd.ai.transform", "AI 文字轉換"),

        ("arg.write.text", "要寫入的文字（省略則從 stdin 讀取）"),
        ("arg.write.tag", "指定標籤名稱"),
        ("arg.write.tag_id", "指定標籤 ID"),
        ("arg.write.raw", "跳過智慧規則"),

        ("arg.paste.index", "面板項目索引（1-9）"),
        ("arg.paste.plain", "純文字貼上"),
        ("arg.paste.target", "指定目標 App（Bundle ID）"),

        ("arg.ai.prompt", "AI 指令（prompt）"),
        ("arg.ai.text", "輸入文字（省略則從 stdin 讀取）"),
        ("arg.ai.save", "自動儲存結果到剪貼簿"),
        ("arg.ai.tag_id", "儲存到指定標籤 ID"),
        ("arg.ai.query", "搜尋關鍵字"),
        ("arg.ai.mode", "搜尋模式"),
        ("arg.ai.limit", "結果數量（預設 10，最大 50）"),
        ("arg.ai.transform_text", "待轉換文字（省略則從 stdin 讀取）"),
        ("arg.ai.plugin", "使用指定外掛 ID"),
        ("arg.completion.shell", "Shell 類型"),

        ("health.ok", "ok — Deck App 連線正常"),
        ("write.ok", "已寫入剪貼簿"),
        ("paste.ok", "已貼上第 {} 項"),
        ("panel.toggled", "面板已切換"),

        ("err.not_running", "Deck App 未執行或未啟用 Deck CLI"),
        ("err.connection", "連線失敗: {}"),
        ("err.auth", "認證失敗: {}"),
        ("err.token_not_found", "Token 檔案不存在: {}"),
        ("err.timeout", "請求逾時"),
        ("err.protocol", "協定錯誤: {}"),
        ("err.server", "伺服器錯誤 [{}]: {}"),
        ("err.io", "IO 錯誤: {}"),
        ("err.token_read", "無法讀取 token 檔案 {}: {}"),
        ("err.token_empty", "token 檔案為空"),
        ("err.conn_closed", "連線已關閉"),
        ("err.auth_rejected", "認證被拒絕"),
        ("err.no_session", "無 session token"),
        ("err.id_mismatch", "回應 ID 不符: 預期 {}，收到 {}"),
        ("err.stdin_hint", "請提供文字參數，或透過管道傳入（如: echo \"text\" | deckclip write）"),
    ])
}
