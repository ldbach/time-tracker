# Time Tracker
Time Tracker is a simple web application to track time spent on tasks. It allows you to start a session, stop it, and view completed sessions with start time, end time, and duration.  

The backend is built with **Rust** and **Axum**, and the frontend is built with **React**.

---

## Features

- Start and stop a timer for work sessions.
- Live duration counter while a session is running.
- View completed sessions with start time, end time, and duration.
- Delete sessions from the session list.
- Simple REST API backend in Rust (Axum).
- Responsive frontend in React.

---

## Installation

Clone the repository:

```bash
git clone https://github.com/ldbach/time-tracker.git
cd time-tracker
```

### Backend

Build and run the backend (requires Rust and Cargo):

```bash
cd time_tracker_backend
cargo run
```

### Frontend

1. Navigate to the frontend folder

```bash
cd time-tracker-backend
```

2. Install dependencies

```bash
npm install
```

3. Run the frontend

```bash
npm start
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)