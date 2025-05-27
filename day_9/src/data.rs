#[derive(Debug, Clone)]
pub enum StorageCell {
    Empty,
    Full(usize),
}

impl StorageCell {
    pub fn new(content: usize) -> Self {
        Self::Full(content)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }

    pub fn get_content_valued(&self, value: usize) -> usize {
        let own_value = match self {
            Self::Empty => 0,
            Self::Full(v) => *v,
        };
        own_value * value
    }
}

#[derive(Debug, Clone)]
pub enum StorageFile {
    Empty { size: usize },
    Full { size: usize, value: usize },
}

impl StorageFile {
    pub fn empty(size: usize) -> Self {
        Self::Empty { size }
    }

    pub fn full(size: usize, value: usize) -> Self {
        Self::Full { size, value }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty { .. } => true,
            Self::Full { .. } => false,
        }
    }

    pub fn can_fit(&self, other_size: usize) -> bool {
        match self {
            Self::Empty { size } => *size >= other_size,
            Self::Full { .. } => false,
        }
    }

    pub fn get_size(&self) -> usize {
        match self {
            Self::Empty { size } => *size,
            Self::Full { size, .. } => *size,
        }
    }

    pub fn get_value(&self) -> usize {
        match self {
            Self::Empty { .. } => usize::MAX,
            Self::Full { value, .. } => *value,
        }
    }

    pub fn split_off(&self, size: usize) -> (Self, Self) {
        let swapped = Self::empty(size);
        let remaining = Self::empty(self.get_size() - size);
        (swapped, remaining)
    }

    pub fn flatten(&self) -> Vec<StorageCell> {
        let template: StorageCell = self.into();
        let mut items = Vec::new();
        for _ in 0..self.get_size() {
            items.push(template.clone());
        }
        items
    }
}

impl From<&StorageFile> for StorageCell {
    fn from(value: &StorageFile) -> Self {
        match value {
            StorageFile::Full { value, .. } => Self::Full(*value),
            StorageFile::Empty { .. } => Self::Empty,
        }
    }
}
