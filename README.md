# We Better Worked
We Better Worked is a learning project to build a time-logging app inspired by WeWorked — but written in Rust.
The goal is to track hours worked on specific customers and projects, and whether the time is billable or not.

This project serves as a hands-on way to learn Rust, while experimenting with backend, frontend, and database concepts.

## Core Learning Goals

- Learn Rust through practical development
- Understand idiomatic project structure in Rust
- Understand idiomatic project structure in Rust
- CLI or Build a frontend using HTMX

### Current Features

### Planned Features
- [ ] User Accounts: Register and login
- [ ] Customer and Project list: CRUD
- [ ] Time Entries: Manually log time
- [ ] Dashboard:See your time logs for the day/week

## Getting Started

## Nice-to-Have (Future)

- CSV export
- Weekly summary email
- Mobile-friendly UI
- Project tags/categories
- OAuth login (Google/GitHub)


## My Notes

-local
export DATABASE_URL=sqlite:///home/gerhard/Dev/we_better_worked/db/we_better_worked
cargo sqlx prepare

-codespaces
export DATABASE_URL=sqlite://$(pwd)/db/we_better_worked.db
cargo sqlx prepare