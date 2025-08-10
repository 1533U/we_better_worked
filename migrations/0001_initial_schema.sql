-- Users' table
CREATE TABLE user_tbl(
    user_id         INTEGER PRIMARY KEY AUTOINCREMENT,
    display_name    TEXT    NOT NULL,
    email           TEXT    NOT NULL,
    active          BOOLEAN NOT NULL DEFAULT 1,
    password_hash   TEXT    NOT NULL,
    creation_date   DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (email)

);

-- Clients' table
CREATE TABLE customer(
    customer_id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name                TEXT NOT NULL,
    email               TEXT NOT NULL,
    active              BOOLEAN NOT NULL DEFAULT 1,

    creation_user_id    INTEGER NOT NULL,
    last_modify_user_id INTEGER NOT NULL,
    creation_date       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modify_date    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (creation_user_id) REFERENCES user_tbl(user_id),
    FOREIGN KEY (last_modify_user_id) REFERENCES user_tbl(user_id),
    UNIQUE (email),
    UNIQUE (name)
);

--Frameworks' table
CREATE TABLE base_framework(
    base_framework_id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name                TEXT    NOT NULL,
    active              BOOLEAN DEFAULT 1,
    github_repo         TEXT        NULL,

    creation_user_id    INTEGER NOT NULL,
    last_modify_user_id INTEGER     NULL,
    last_modify_date    DATETIME DEFAULT CURRENT_TIMESTAMP,
    creation_date       DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (creation_user_id) REFERENCES user_tbl(user_id),
    FOREIGN KEY (last_modify_user_id) REFERENCES user_tbl(user_id),
    UNIQUE (name)
);

CREATE TABLE framework_version(
    framework_version_id   INTEGER PRIMARY KEY AUTOINCREMENT,
    base_framework_id      INTEGER NOT NULL,
    version                TEXT    NOT NULL,
    active                 BOOLEAN NOT NULL DEFAULT 1,

    creation_user_id    INTEGER NOT NULL,
    last_modify_user_id INTEGER     NULL,
    last_modify_date    DATETIME DEFAULT CURRENT_TIMESTAMP,
    creation_date       DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (creation_user_id) REFERENCES user_tbl(user_id),
    FOREIGN KEY (last_modify_user_id) REFERENCES user_tbl(user_id),

    FOREIGN KEY (base_framework_id) REFERENCES base_framework(base_framework_id),
    UNIQUE (base_framework_id,version)
);

--solutions' table
CREATE TABLE solution(
    solution_id             INTEGER PRIMARY KEY AUTOINCREMENT,
    name                            TEXT    NOT NULL,
    customer_id                     INTEGER NOT NULL,
    url                             TEXT    NOT NULL,
    os_connect_version_id           INTEGER NOT NULL,
    portal_version_id               INTEGER NOT NULL,
    os_warehouse_version_id         INTEGER NOT NULL,
    version                         TEXT    NOT NULL,
    hosted                          TEXT        NULL,
    requires_vpn                    BOOLEAN     NULL,

    creation_user_id    INTEGER NOT NULL,
    last_modify_user_id INTEGER     NULL,
    last_modify_date    DATETIME DEFAULT CURRENT_TIMESTAMP,
    creation_date       DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (creation_user_id) REFERENCES user_tbl(user_id),
    FOREIGN KEY (last_modify_user_id) REFERENCES user_tbl(user_id),

    FOREIGN KEY (os_connect_version_id) REFERENCES framework_version(framework_version_id),
    FOREIGN KEY (portal_version_id) REFERENCES framework_version(framework_version_id),
    FOREIGN KEY (os_warehouse_version_id) REFERENCES framework_version(framework_version_id),
    FOREIGN KEY (customer_id) REFERENCES customer(customer_id),
    UNIQUE (url)
);

CREATE TABLE solution_module(
    solution_module_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name                TEXT    NOT NULL,
    description         TEXT    NOT NULL,
    solution_id         INTEGER NOT NULL,
    github_repo         TEXT        NULL,

    creation_user_id    INTEGER NOT NULL,
    last_modify_user_id INTEGER     NULL,
    last_modify_date    DATETIME DEFAULT CURRENT_TIMESTAMP,
    creation_date       DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (creation_user_id) REFERENCES user_tbl(user_id),
    FOREIGN KEY (last_modify_user_id) REFERENCES user_tbl(user_id),

    FOREIGN KEY (solution_id) REFERENCES solution(solution_id)
);

--Projects
CREATE TABLE project(
    project_id      INTEGER PRIMARY KEY AUTOINCREMENT,
    name            TEXT NOT NULL,
    description     TEXT     NULL,
    customer_id     INTEGER NOT NULL,

    creation_user_id    INTEGER NOT NULL,
    last_modify_user_id INTEGER     NULL,
    last_modify_date    DATETIME DEFAULT CURRENT_TIMESTAMP,
    creation_date       DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (creation_user_id) REFERENCES user_tbl(user_id),
    FOREIGN KEY (last_modify_user_id) REFERENCES user_tbl(user_id),

    FOREIGN KEY (customer_id) REFERENCES customer(customer_id)
);

--Features
CREATE TABLE feature(
    feature_id  INTEGER PRIMARY KEY AUTOINCREMENT,
    name            TEXT    NOT NULL,
    project_id      INTEGER NOT NULL,
    description     TEXT        NULL,

    solution_module_id      INTEGER NULL,
    framework_version_id    INTEGER NULL,

    creation_user_id    INTEGER NOT NULL,
    last_modify_user_id INTEGER     NULL,
    last_modify_date    DATETIME DEFAULT CURRENT_TIMESTAMP,
    creation_date       DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (creation_user_id) REFERENCES user_tbl(user_id),
    FOREIGN KEY (last_modify_user_id) REFERENCES user_tbl(user_id),

    FOREIGN KEY (project_id) REFERENCES project(project_id),
    FOREIGN KEY (solution_module_id) REFERENCES solution_module(solution_module_id),
    FOREIGN KEY (framework_version_id) REFERENCES framework_version(framework_version_id),
    CHECK (solution_module_id IS NULL OR framework_version_id IS NULL)

);

--tasks
CREATE TABLE task(
    task_id                 INTEGER PRIMARY KEY AUTOINCREMENT,
    name                    TEXT    NOT NULL,
    customer_id             INTEGER NOT NULL,
    project_id              INTEGER NOT NULL,
    feature_id              INTEGER     NULL,
    description             TEXT        NULL,

    solution_module_id      INTEGER NULL,
    framework_version_id    INTEGER NULL,

    creation_user_id        INTEGER NOT NULL,
    last_modify_user_id     INTEGER NULL,
    last_modify_date        DATETIME DEFAULT CURRENT_TIMESTAMP,
    creation_date           DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (creation_user_id) REFERENCES user_tbl(user_id),
    FOREIGN KEY (last_modify_user_id) REFERENCES user_tbl(user_id),
    FOREIGN KEY (project_id) REFERENCES project(project_id),
    FOREIGN KEY (feature_id) REFERENCES feature(feature_id),
    FOREIGN KEY (solution_module_id) REFERENCES solution_module(solution_module_id),
    FOREIGN KEY (framework_version_id) REFERENCES framework_version(framework_version_id),
    FOREIGN KEY (customer_id) REFERENCES customer(customer_id),
    CHECK (solution_module_id IS NULL OR framework_version_id IS NULL)
);

CREATE TABLE we_worked(
    we_worked_id    INTEGER PRIMARY KEY AUTOINCREMENT,
    worker_user_id  INTEGER NOT NULL,
    date            DATE    NOT NULL,
    effort_minutes  INTEGER NOT NULL,
    project_id      INTEGER NOT NULL,
    feature_id      INTEGER     NULL,
    task_id         INTEGER     NULL,
    is_billable      BOOLEAN NOT NULL DEFAULT 1,

    creation_user_id        INTEGER NOT NULL,
    last_modify_user_id     INTEGER NULL,
    last_modify_date        DATETIME DEFAULT CURRENT_TIMESTAMP,
    creation_date           DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (creation_user_id) REFERENCES user_tbl(user_id),
    FOREIGN KEY (last_modify_user_id) REFERENCES user_tbl(user_id),
    FOREIGN KEY (project_id) REFERENCES project(project_id),
    FOREIGN KEY (feature_id) REFERENCES feature(feature_id),
    FOREIGN KEY (task_id) REFERENCES task(task_id),
    FOREIGN KEY (worker_user_id) REFERENCES user_tbl(user_id),
    CHECK (feature_id IS  NULL OR task_id IS  NULL)

);

--todo custom_fields table
--todo custom_field_options table
--todo custom_field_value table


