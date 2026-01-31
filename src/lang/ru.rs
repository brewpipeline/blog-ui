use chrono::Locale;

pub const DATE_LOCALE: Locale = Locale::ru_RU;

// Common
pub const COMMON_PAGE_NOT_FOUND_TITLE: &str = "Cтраница не найдена";
pub const COMMON_PAGE_NOT_FOUND_TEXT: &str = "Cтраница не найдена!";
pub const COMMON_LOADING: &str = "Загрузка...";
pub const COMMON_SAVE: &str = "Сохранить";
pub const COMMON_DELETE: &str = "Удалить";
pub const COMMON_SEND: &str = "Отправить";

// Navigation
pub const NAV_POSTS: &str = "Публикации";
pub const NAV_AUTHORS: &str = "Авторы";
pub const NAV_NEW_POST: &str = "Новая публикация";
pub const NAV_UNPUBLISHED: &str = "Неопубликованное";
pub const NAV_HIDDEN: &str = "Скрытое";
pub const NAV_GO_TO_CHANNEL: &str = "Перейти в канал";

// Footer
pub const FOOTER_MENU: &str = "Меню";
pub const FOOTER_FEED: &str = "Лента";
pub const FOOTER_FEED_ARIA: &str = "Лента";
pub const FOOTER_INFO: &str = "Инфо";

// Auth
pub const AUTH_LOGIN: &str = "Войти";
pub const AUTH_LOGIN_TITLE: &str = "Войти";
pub const AUTH_LOGIN_ARIA: &str = "Войти";
pub const AUTH_LOGIN_VIA_TELEGRAM_TITLE: &str = "Войти через Telegram";
pub const AUTH_LOGIN_VIA_TELEGRAM_ARIA: &str = "Войти через Telegram";
pub const AUTH_LOGIN_ERROR: &str = "Ошибка авторизации: ";
pub const AUTH_USERNAME: &str = "Имя пользователя";
pub const AUTH_PASSWORD: &str = "Пароль";
pub const AUTH_AUTHORIZED: &str = "Авторизован!";
pub const AUTH_OR: &str = "ИЛИ";
pub const AUTH_LOGOUT_TITLE: &str = "Выход";
pub const AUTH_LOGOUT_CONFIRM: &str = "Вы точно хотите выйти?";
pub const AUTH_LOGOUT: &str = "Выйти";
pub const AUTH_NOT_AUTHORIZED: &str = "Неавторизован!";
pub const AUTH_PROFILE: &str = "Профиль";
pub const AUTH_MY_UNPUBLISHED: &str = "Неопубликованное";
pub const AUTH_SETTINGS: &str = "Настройки";
pub const AUTH_LOGOUT_MENU: &str = "Выход";

// Posts page
pub const POSTS_TITLE: &str = "Публикации";
pub const POSTS_ERROR: &str = "Ошибка загрузки публикаций!";
pub const POSTS_EMPTY: &str = "Нет публикаций.";

// Post page
pub const POST_TITLE: &str = "Публикация";
pub const POST_LINK_BROKEN_TITLE: &str = "Ссылка на публикацию повреждена";
pub const POST_LINK_BROKEN_TEXT: &str = "Ссылка на публикацию повреждена!";
pub const POST_ERROR_TITLE: &str = "Ошибка загрузки публикации";
pub const POST_ERROR_TEXT: &str = "Ошибка загрузки публикации!";
pub fn post_meta_title(title: &str) -> String {
    format!("{} - Публикация", title)
}

// Authors page
pub const AUTHORS_TITLE: &str = "Авторы";
pub const AUTHORS_ERROR: &str = "Ошибка загрузки авторов!";
pub const AUTHORS_EMPTY: &str = "Нет авторов.";

// Author page
pub const AUTHOR_TITLE: &str = "Автор";
pub const AUTHOR_POSTS: &str = "Публикации автора ";
pub const AUTHOR_POSTS_ERROR: &str = "Ошибка загрузки публикаций автора!";
pub const AUTHOR_POSTS_EMPTY: &str = "У автора нет публикаций.";
pub const AUTHOR_ERROR_TITLE: &str = "Ошибка загрузки автора";
pub const AUTHOR_ERROR_TEXT: &str = "Ошибка загрузки автора!";
pub fn author_meta_title(name: &str) -> String {
    format!("{} - Автор", name)
}

// Author card
pub const AUTHOR_CARD_BAN: &str = " Бан ";
pub const AUTHOR_CARD_UNBAN: &str = " Разбан ";
pub const AUTHOR_CARD_NAME_HIDDEN: &str = "(Имя скрыто)";
pub const AUTHOR_CARD_NAME_MISSING: &str = "(Имя не указано)";
pub const AUTHOR_CARD_EDITOR: &str = "Главный редактор";
pub const AUTHOR_CARD_BLOCKED: &str = "Заблокирован";
pub const AUTHOR_CARD_STATUS_HIDDEN: &str = "(Информация о себе скрыта)";
pub const AUTHOR_CARD_STATUS_MISSING: &str = "(Информация о себе отсутствует)";

// Search
pub const SEARCH_POSTS_PLACEHOLDER: &str = "Поиск публикаций...";
pub const SEARCH_AUTHORS_PLACEHOLDER: &str = "Поиск авторов...";
pub const SEARCH_POSTS_TITLE: &str = "Поиск публикаций";
pub const SEARCH_AUTHORS_TITLE: &str = "Поиск авторов";
pub const SEARCH_POSTS_ERROR: &str = "Ошибка загрузки результатов поиска публикаций!";
pub const SEARCH_POSTS_EMPTY: &str = "Публикаций не найдено!";
pub const SEARCH_POSTS_HINT: &str = "Начните ввод для поиска публикаций...";
pub const SEARCH_AUTHORS_ERROR: &str = "Ошибка загрузки результатов поиска авторов!";
pub const SEARCH_AUTHORS_EMPTY: &str = "Авторов не найдено!";
pub const SEARCH_AUTHORS_HINT: &str = "Начните ввод для поиска авторов...";
pub const SEARCH_BUTTON_TITLE: &str = "Поиск";

// Tag page
pub const TAG_TITLE: &str = "Тег";
pub const TAG_PREFIX: &str = "Тег: ";
pub const TAG_LINK_BROKEN_TITLE: &str = "Ссылка на тег повреждена";
pub const TAG_LINK_BROKEN_TEXT: &str = "Ссылка на тег повреждена!";
pub const TAG_POSTS_ERROR: &str = "Ошибка загрузки публикаций по тегу!";
pub const TAG_POSTS_EMPTY: &str = "Нет публикаций по тегу.";
pub const TAG_ERROR_TITLE: &str = "Ошибка загрузки тега";
pub const TAG_ERROR_TEXT: &str = "Ошибка загрузки тега!";
pub fn tag_meta_title(title: &str) -> String {
    format!("{} - Тег", title)
}

// Unpublished posts
pub const UNPUB_TITLE: &str = "Неопубликованное";
pub const UNPUB_AUTH_REQUIRED: &str = "Нужна авторизация для просмотра неопубликованного!";
pub const UNPUB_EDITORS_ONLY: &str = "Просмотр неопубликованного доступен только редакторам!";
pub const UNPUB_ERROR: &str = "Ошибка загрузки неопубликованного!";
pub const UNPUB_EMPTY: &str = "Нет неопубликованного.";

// Hidden posts
pub const HIDDEN_TITLE: &str = "Скрытое";
pub const HIDDEN_AUTH_REQUIRED: &str = "Нужна авторизация для просмотра скрытого!";
pub const HIDDEN_EDITORS_ONLY: &str = "Просмотр скрытого доступен только редакторам!";
pub const HIDDEN_ERROR: &str = "Ошибка загрузки скрытого!";
pub const HIDDEN_EMPTY: &str = "Нет скрытого.";

// My unpublished posts
pub const MY_UNPUB_TITLE: &str = "Мое неопубликованное";
pub const MY_UNPUB_AUTH_REQUIRED: &str = "Нужна авторизация для получения моего неопубликованного!";
pub const MY_UNPUB_ERROR: &str = "Ошибка загрузки моего неопубликованного!";
pub const MY_UNPUB_EMPTY: &str = "Нет моего неопубликованного.";

// Edit post
pub const EDIT_POST_NEW_TITLE: &str = "Новая публикация";
pub const EDIT_POST_EDIT_TITLE: &str = "Редактирование публикации";
pub const EDIT_POST_NEW_AUTH: &str = "Создавать публикации можно только авторизованным авторам!";
pub const EDIT_POST_EDIT_AUTH: &str =
    "Редактировать публикации можно только авторизованным авторам!";
pub const EDIT_POST_DELETED: &str = "Публикация удалена!";
pub const EDIT_POST_EDITING: &str = "Редактирование публикации: ";
pub const EDIT_POST_IMAGE_LABEL: &str = "Изображение (Cсылка) (Опциональное)";
pub const EDIT_POST_IMAGE_PLACEHOLDER: &str = "Что-то визуально приятное...";
pub const EDIT_POST_TITLE_LABEL: &str = "Заголовок";
pub const EDIT_POST_TITLE_PLACEHOLDER: &str = "Что-то захватывающее внимание...";
pub const EDIT_POST_TITLE_VALIDATION: &str =
    "Пожалуйста, введите заголовок публикации, это обязательное поле!";
pub const EDIT_POST_SUMMARY_LABEL: &str = "Короткая версия";
pub const EDIT_POST_SUMMARY_PLACEHOLDER: &str = "Что-то короткое, но важное!";
pub const EDIT_POST_SUMMARY_VALIDATION: &str =
    "Пожалуйста, введите короткую версию публикации, это обязательное поле!";
pub const EDIT_POST_CONTENT_LABEL: &str = "Полная версия (Опциональное)";
pub const EDIT_POST_CONTENT_PLACEHOLDER: &str = "Что-то динное и скучн... веселое!";
pub const EDIT_POST_TAGS_LABEL: &str = "Теги (через `,`) (Опциональное)";
pub const EDIT_POST_TAGS_PLACEHOLDER: &str = "Что-то напоминающее о...";
pub const EDIT_POST_UNPUBLISHED: &str = "Неопубликовано";
pub const EDIT_POST_PUBLISHED: &str = "Опубликовано";
pub const EDIT_POST_HIDDEN_STATUS: &str = "Скрыто";
pub const EDIT_POST_BLOCKED: &str = "Вы заблокированы!";
pub const EDIT_POST_ONLY_AUTHOR_OR_EDITOR: &str =
    "Только автор или редактор может редактировать публикацию!";
pub const EDIT_POST_LOADING: &str = "Загрузка публикации для редактирования...";
pub const EDIT_POST_LOAD_ERROR: &str = "Ошибка загрузки публикации для редактирования!";
pub fn edit_post_add_error(message: &str) -> String {
    format!("Ошибка добавления публикации: {}", message)
}
pub fn edit_post_edit_error(message: &str) -> String {
    format!("Ошибка редактирования публикации: {}", message)
}
pub fn edit_post_delete_error(message: &str) -> String {
    format!("Ошибка удаления публикации: {}", message)
}
pub const TINYMCE_LANG: &str = "ru";

// Settings
pub const SETTINGS_TITLE: &str = "Настройки";
pub const SETTINGS_AUTH_REQUIRED: &str = "Настройки доступны только авторизованным авторам!";
pub const SETTINGS_PRIMARY_TITLE: &str = "Основные данные профиля";
pub const SETTINGS_PRIMARY_RESET_TITLE: &str = "Сбросить основные данные";
pub const SETTINGS_DATA_UPDATED: &str = "Данные успешно обновлены: ";
pub const SETTINGS_DATA_ERROR: &str = "Ошибка обновления данных: ";
pub const SETTINGS_USE_TELEGRAM: &str =
    "Использовать данные Telegram (используйте кнопку ниже, чтобы выбрать этот пункт)";
pub const SETTINGS_SYNC_HINT: &str = "Также используйте кнопку для синхронизации данныx.";
pub const SETTINGS_USE_CUSTOM: &str = "Использовать пользовательские данные";
pub const SETTINGS_SLUG: &str = "Имя профиля (уникальное)";
pub const SETTINGS_IMAGE_URL: &str = "Изображение профиля (ссылка)";
pub const SETTINGS_FIRST_NAME: &str = "Имя";
pub const SETTINGS_LAST_NAME: &str = "Фамилия";
pub const SETTINGS_SECONDARY_TITLE: &str = "Второстепенные данные профиля";
pub const SETTINGS_SECONDARY_RESET_TITLE: &str = "Сбросить второстепенные данные";
pub const SETTINGS_ABOUT: &str = "О себе";
pub const SETTINGS_EMAIL: &str = "Почта";
pub const SETTINGS_PHONE: &str = "Телефон";
pub const SETTINGS_BUTTON_NOT_READY: &str = "Кнопка еще разрабатывается...";

// Comments
pub const COMMENT_TITLE: &str = "Комментарии";
pub const COMMENT_ERROR: &str = "Ошибка загрузки комментариев!";
pub const COMMENT_EMPTY: &str = "Нет комментариев.";
pub const COMMENT_PLACEHOLDER: &str = "Комментарий...";
pub const COMMENT_DELETE_TITLE: &str = "Удалить комментарий";
pub const COMMENT_DELETING: &str = "Удаление...";
pub const COMMENT_DELETED: &str = "Удален!";
pub const COMMENT_WAS_DELETED: &str = "Комментарий удален.";

// Post card
pub const POSTCARD_EDIT_TITLE: &str = "Редактировать публикацию";
pub const POSTCARD_UNPUBLISHED_TITLE: &str = "Неопубликовано";
pub const POSTCARD_HIDDEN_TITLE: &str = "Скрыто";
pub const POSTCARD_STAR_ADD: &str = "Добавить в рекомендации";
pub const POSTCARD_STAR_REMOVE: &str = "Убрать из рекомендаций";

// ChatGPT
pub const CHATGPT_GREETING: &str = "Привет! Я — ChatGPT, адаптированный под этот блог. Я в курсе свежих публикаций и помогу подобрать интересное. О чём бы ты хотел почитать?";
pub const CHATGPT_USER: &str = "Вы";
pub const CHATGPT_SYSTEM: &str = "Система";
pub const CHATGPT_TYPING: &str = "Печатает…";
pub const CHATGPT_PLACEHOLDER: &str = "Спросите что-нибудь…";
pub const CHATGPT_UNKNOWN_REASON: &str = "неизвестная причина";
pub const CHATGPT_NETWORK_ERROR: &str = "Произошла ошибка сети при получении ответа";
pub fn chatgpt_error(reason: &str) -> String {
    format!("Произошла ошибка при получении ответа: {}", reason)
}

// Subscribe button
pub const SUBSCRIBE_SUBSCRIBED: &str = "Вы подписаны на уведомления";
pub const SUBSCRIBE_UNSUBSCRIBED: &str = "Вы отписаны от уведомлений";

// Recommended post
pub const RECOMMENDED_TITLE: &str = "Вам будет интересно";

// Body
pub const BODY_FEED_ARIA: &str = "Лента";
pub const BODY_FEED_TITLE: &str = "Лента";
pub const BODY_INFO_ARIA: &str = "Информация";
pub const BODY_INFO_TITLE: &str = "Информация";
pub const BODY_RULES: &str = "Правила";
pub const BODY_ABOUT: &str = "О Tikitko";
