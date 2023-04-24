CREATE TABLE cron_jobs (
  id INT NOT NULL AUTO_INCREMENT,
  name VARCHAR(255) NOT NULL,
  schedule VARCHAR(255) NOT NULL,
  payload JSON NOT NULL,
  job_type VARCHAR(255) NOT NULL,
  last_run_at DATETIME DEFAULT NULL,
  last_run_status ENUM('Success', 'Failure', 'Aborted', 'Skipped', 'Running', 'TimedOut', 'Paused', 'Unknown') DEFAULT 'Unknown',
  message TEXT,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (id)
);

CREATE INDEX idx_cron_jobs_schedule ON cron_jobs (schedule);
