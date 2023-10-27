use chrono::{DateTime, Utc};
use sqlx::{query_scalar, FromRow, PgConnection};
use ulx_notary_primitives::NotebookNumber;

use crate::{ensure, error::Error};

pub struct NotebookStatusStore;

#[derive(Copy, Clone)]
pub enum NotebookFinalizationStep {
	Open = 1,
	ReadyForClose = 2,
	Closed = 3,
	GetAuditors = 4,
	Audited = 5,
	Submitted = 6,
	Finalized = 7,
}
#[derive(FromRow)]
pub struct NotebookStatusRow {
	pub notebook_number: NotebookNumber,
	pub chain_transfers: i32,
	pub step: NotebookFinalizationStep,
	pub open_time: DateTime<Utc>,
	pub ready_for_close_time: Option<DateTime<Utc>>,
	pub closed_time: Option<DateTime<Utc>>,
	pub get_auditors_time: Option<DateTime<Utc>>,
	pub audited_time: Option<DateTime<Utc>>,
	pub submitted_time: Option<DateTime<Utc>>,
	pub finalized_time: Option<DateTime<Utc>>,
}

impl NotebookStatusStore {
	pub async fn lock_to_stop_appends<'a>(
		db: impl sqlx::PgExecutor<'a> + 'a,
		notebook_number: NotebookNumber,
	) -> anyhow::Result<(), Error> {
		sqlx::query!(
			r#"
			SELECT 1 as exists FROM notebook_status WHERE notebook_number = $1 AND step = $2 FOR UPDATE NOWAIT LIMIT 1
			"#,
			notebook_number as i32,
			NotebookFinalizationStep::Open as i32
		)
			.fetch_one(db)
			.await?;
		Ok(())
	}
	pub async fn lock_latest_for_appending<'a>(
		db: &mut PgConnection,
	) -> anyhow::Result<u32, Error> {
		for _ in 0..3 {
			let row = query_scalar!(
				r#"SELECT notebook_number FROM notebook_status WHERE step=$1 FOR SHARE LIMIT 1"#,
				NotebookFinalizationStep::Open as i32
			)
			.fetch_optional(&mut *db)
			.await?;
			if let Some(row) = row {
				return Ok(row as u32)
			}
			tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
		}
		Err(Error::InternalError("Unable to lock notebook for write".to_string()))
	}

	pub async fn lock_with_step<'a>(
		db: impl sqlx::PgExecutor<'a> + 'a,
		step: NotebookFinalizationStep,
	) -> anyhow::Result<Option<u32>, Error> {
		let result = sqlx::query!(
			r#"
				SELECT notebook_number FROM notebook_status WHERE step=$1 FOR UPDATE NOWAIT LIMIT 1
			"#,
			step as i32
		)
		.fetch_optional(db)
		.await?;
		if let Some(row) = result {
			return Ok(Some(row.notebook_number as u32))
		}
		Ok(None)
	}

	pub async fn create<'a>(
		db: impl sqlx::PgExecutor<'a> + 'a,
		notebook_number: NotebookNumber,
	) -> anyhow::Result<(), Error> {
		let res = sqlx::query!(
			r#"
				INSERT INTO notebook_status (notebook_number, open_time, step) VALUES ($1, now(), $2)
			"#,
			notebook_number as i32,
			NotebookFinalizationStep::Open as i32,
		)
		.execute(db)
		.await?;
		ensure!(
			res.rows_affected() == 1,
			Error::InternalError("Unable to insert notebook status".to_string())
		);
		Ok(())
	}
	pub async fn increment_chain_transfers<'a>(
		db: &mut PgConnection,
		notebook_number: NotebookNumber,
		max_transfer_per_notebook: u32,
	) -> anyhow::Result<(), Error> {
		let result = sqlx::query!(
			r#"
				UPDATE notebook_status SET chain_transfers = chain_transfers + 1 
				WHERE notebook_number = $1 AND chain_transfers < $2
			"#,
			notebook_number as i32,
			max_transfer_per_notebook as i32,
		)
		.execute(db)
		.await?;

		ensure!(result.rows_affected() == 1, Error::MaxNotebookChainTransfersReached);
		Ok(())
	}

	pub async fn step_up_expired_open<'a>(
		db: &mut PgConnection,
	) -> anyhow::Result<Option<u32>, Error> {
		let result = sqlx::query!(
			r#"
				SELECT * FROM notebook_status 
				WHERE step = $1 AND open_time < $2
				ORDER BY open_time ASC 
				LIMIT 1
			"#,
			NotebookFinalizationStep::Open as i32,
			Utc::now() - chrono::Duration::minutes(1),
		)
		.fetch_optional(&mut *db)
		.await?;

		if let Some(result) = result {
			let notebook_number = result.notebook_number as u32;
			Self::next_step(&mut *db, notebook_number, NotebookFinalizationStep::Open).await?;
			return Ok(Some(notebook_number))
		}
		Ok(None)
	}

	pub async fn next_step<'a>(
		db: impl sqlx::PgExecutor<'a> + 'a,
		notebook_number: NotebookNumber,
		current_step: NotebookFinalizationStep,
	) -> anyhow::Result<(), Error> {
		let (next_step, time_field) = match current_step {
			NotebookFinalizationStep::Open =>
				(NotebookFinalizationStep::ReadyForClose, "open_time"),
			NotebookFinalizationStep::ReadyForClose =>
				(NotebookFinalizationStep::Closed, "ready_for_close_time"),
			NotebookFinalizationStep::Closed =>
				(NotebookFinalizationStep::GetAuditors, "closed_time"),
			NotebookFinalizationStep::GetAuditors =>
				(NotebookFinalizationStep::Audited, "get_auditors_time"),
			NotebookFinalizationStep::Audited =>
				(NotebookFinalizationStep::Submitted, "audited_time"),
			NotebookFinalizationStep::Submitted =>
				(NotebookFinalizationStep::Finalized, "submitted_time"),
			NotebookFinalizationStep::Finalized => return Ok(()),
		};

		let res = sqlx::query(&*format!(
			r#"
				UPDATE notebook_status SET step=$1, {}=now() WHERE notebook_number=$2 AND step=$3
			"#,
			&*time_field
		))
		.bind(next_step as i32)
		.bind(notebook_number as i32)
		.bind(current_step as i32)
		.execute(db)
		.await?;
		ensure!(
			res.rows_affected() == 1,
			Error::InternalError("Unable to update notebook step".to_string())
		);
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use frame_support::assert_ok;
	use futures::future::try_join;
	use sqlx::PgPool;

	use crate::{
		error::Error,
		stores::notebook_status::{NotebookFinalizationStep, NotebookStatusStore},
	};

	#[sqlx::test]
	async fn test_locks(pool: PgPool) -> anyhow::Result<()> {
		let _ = tracing_subscriber::fmt::try_init();
		let notebook_number = 1;
		{
			let mut tx = pool.begin().await?;

			let _ = NotebookStatusStore::create(&mut *tx, 1).await?;

			tx.commit().await?;
		}
		{
			let mut tx1 = pool.begin().await?;
			let mut tx2 = pool.begin().await?;
			assert_eq!(
				NotebookStatusStore::lock_latest_for_appending(&mut *tx1).await?,
				notebook_number
			);
			assert_eq!(
				NotebookStatusStore::lock_latest_for_appending(&mut *tx2).await?,
				notebook_number
			);

			let mut fail_tx = pool.begin().await?;
			assert!(NotebookStatusStore::lock_to_stop_appends(&mut *fail_tx, notebook_number)
				.await
				.is_err());
			fail_tx.commit().await?;

			tx1.commit().await?;
			tx2.commit().await?;

			let (rx, txer) = tokio::sync::oneshot::channel();

			let cloned = pool.clone();
			let task1 = tokio::spawn(async move {
				let mut tx = cloned.begin().await?;
				assert_ok!(
					NotebookStatusStore::lock_to_stop_appends(&mut *tx, notebook_number).await
				);
				let _ = rx.send(0);
				// wait for 500 ms
				tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
				NotebookStatusStore::next_step(
					&mut *tx,
					notebook_number,
					NotebookFinalizationStep::Open,
				)
				.await?;
				NotebookStatusStore::create(&mut *tx, 2).await?;
				tx.commit().await?;
				Result::<(), Error>::Ok(())
			});

			let cloned2 = pool.clone();
			let task2 = tokio::spawn(async move {
				let mut tx = cloned2.begin().await?;
				let _ = txer.await;
				let next_notebook =
					NotebookStatusStore::lock_latest_for_appending(&mut *tx).await?;
				tx.commit().await?;
				Result::<u32, Error>::Ok(next_notebook)
			});

			let results = try_join(task1, task2).await?;
			assert_eq!(results.1?, 2);
		}
		Ok(())
	}

	#[sqlx::test]
	async fn test_max_chain_transfers(pool: PgPool) -> anyhow::Result<()> {
		let mut tx = pool.begin().await?;

		let _ = NotebookStatusStore::create(&mut *tx, 1).await?;
		assert_ok!(NotebookStatusStore::increment_chain_transfers(&mut *tx, 1, 3).await);
		tx.commit().await?;
		let mut tx = pool.begin().await?;
		assert_ok!(NotebookStatusStore::increment_chain_transfers(&mut *tx, 1, 3).await);
		tx.commit().await?;
		let mut tx = pool.begin().await?;
		assert_ok!(NotebookStatusStore::increment_chain_transfers(&mut *tx, 1, 3).await);
		tx.commit().await?;
		let mut tx = pool.begin().await?;

		assert!(matches!(
			NotebookStatusStore::increment_chain_transfers(&mut *tx, 1, 3).await,
			Err(Error::MaxNotebookChainTransfersReached)
		));

		tx.commit().await?;
		Ok(())
	}

	#[sqlx::test]
	async fn test_locks_step(pool: PgPool) -> anyhow::Result<()> {
		let _ = tracing_subscriber::fmt::try_init();
		{
			let mut tx = pool.begin().await?;

			let _ = NotebookStatusStore::create(&mut *tx, 1).await?;
			assert_eq!(
				NotebookStatusStore::lock_with_step(&mut *tx, NotebookFinalizationStep::Open)
					.await?,
				Some(1)
			);

			tx.commit().await?;
		}

		let mut tx = pool.begin().await?;
		assert_eq!(
			NotebookStatusStore::lock_with_step(&mut *tx, NotebookFinalizationStep::Open).await?,
			Some(1)
		);
		{
			let mut tx2 = pool.begin().await?;
			assert!(matches!(
				NotebookStatusStore::lock_with_step(&mut *tx2, NotebookFinalizationStep::Open)
					.await,
				Err(_)
			));
			tx2.rollback().await?;
		}

		assert_ok!(
			NotebookStatusStore::next_step(&mut *tx, 1, NotebookFinalizationStep::Open).await
		);
		tx.commit().await?;
		Ok(())
	}

	#[sqlx::test]
	async fn test_expire_open(pool: PgPool) -> anyhow::Result<()> {
		let mut tx = pool.begin().await?;

		let _ = NotebookStatusStore::create(&mut *tx, 1).await?;
		assert_eq!(NotebookStatusStore::step_up_expired_open(&mut *tx).await?, None);
		tx.commit().await?;

		sqlx::query("update notebook_status set open_time = now() - interval '2 minutes' where notebook_number = 1")
			.execute(&pool)
			.await?;

		let mut tx = pool.begin().await?;
		assert_eq!(NotebookStatusStore::step_up_expired_open(&mut *tx).await?, Some(1));

		Ok(())
	}
}