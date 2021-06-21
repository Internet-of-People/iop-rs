use super::*;

#[derive(Debug, Clone)]
struct TimeSeriesPoint<T> {
    height: BlockHeight,
    value: T,
}

#[derive(Debug, Clone)]
pub struct TimeSeries<T: fmt::Display + PartialEq<T>> {
    initial_value: T,
    points: Vec<TimeSeriesPoint<T>>,
}

impl<T: fmt::Display + PartialEq<T>> TimeSeries<T> {
    pub fn new(initial_value: T) -> Self {
        Self { initial_value, points: Default::default() }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Option<BlockHeight>, &T)> {
        std::iter::once((None, &self.initial_value))
            .chain(self.points.iter().map(|p| (Some(p.height), &p.value)))
    }

    pub fn get(&self, height: BlockHeight) -> &T {
        self.points
            .iter()
            .rev()
            .find(|p| p.height <= height)
            .map(|p| &p.value)
            .unwrap_or_else(|| &self.initial_value)
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    pub fn latest_value(&self) -> &T {
        self.points.last().map(|p| &p.value).unwrap_or_else(|| &self.initial_value)
    }

    pub fn latest_height(&self) -> Option<BlockHeight> {
        self.points.last().map(|p| p.height)
    }

    pub fn apply<D: fmt::Display>(
        &mut self, height: BlockHeight, value: T, context: impl FnOnce() -> D,
    ) -> Result<()> {
        if let Some(last) = self.points.last() {
            ensure!(last.height < height, "{} was already set at height {}", context(), height);
        }
        ensure!(
            self.latest_value() != &value,
            "{} was already set to {} at height {}",
            context(),
            value,
            height
        );
        self.points.push(TimeSeriesPoint { height, value });
        Ok(())
    }

    pub fn revert<D: fmt::Display>(
        &mut self, height: BlockHeight, value: T, context: impl FnOnce() -> D,
    ) -> Result<()> {
        if let Some(last) = self.points.pop() {
            ensure!(
                last.height == height,
                "{} was set at height {}, cannot unset at height {}",
                context(),
                last.height,
                height
            );
            ensure!(
                last.value == value,
                "{} was set to {} at height {}, cannot unset it from {}",
                context(),
                last.value,
                last.height,
                value
            );
            Ok(())
        } else {
            bail!("{} has nothing to unset", context());
        }
    }
}
