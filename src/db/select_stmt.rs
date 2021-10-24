//! SELECT语句

/// SELECT语句
pub struct SelectStmt<'a> {
    table: &'a str,
    fields: &'a str,
    condition: Option<&'a str>,
    order: Option<&'a str>,
    limit: Option<u8>,
    offset: Option<u32>,
}

impl<'a> SelectStmt<'a> {
    fn empty() -> Self {
        Self {
            table: "",
            fields: "*",
            condition: None,
            order: None,
            limit: None,
            offset: None,
        }
    }
    pub fn builder() -> Self {
        Self::empty()
    }
    pub fn table(&mut self, table: &'a str) -> &mut Self {
        self.table = table;
        self
    }
    pub fn fields(&mut self, fields: &'a str) -> &mut Self {
        self.fields = fields;
        self
    }
    pub fn condition(&mut self, condition: Option<&'a str>) -> &mut Self {
        self.condition = condition;
        self
    }
    pub fn order(&mut self, order: Option<&'a str>) -> &mut Self {
        self.order = order;
        self
    }
    pub fn limit(&mut self, limit: Option<u8>) -> &mut Self {
        self.limit = limit;
        self
    }
    pub fn offset(&mut self, offset: Option<u32>) -> &mut Self {
        self.offset = offset;
        self
    }
    pub fn build(&self) -> String {
        self.str()
    }
    pub fn str(&self) -> String {
        format!(
            "SELECT {} FROM {}{}{}{}{}",
            self.fields,
            self.table,
            match self.condition {
                Some(condition) => format!(" WHERE {}", condition),
                None => "".to_owned(),
            },
            match self.order {
                Some(order) => format!(" ORDER BY {}", order),
                None => "".to_owned(),
            },
            match self.limit {
                Some(limit) => format!(" LIMIT {}", limit),
                None => "".to_owned(),
            },
            match self.offset {
                Some(offset) => format!(" OFFSET {}", offset),
                None => "".to_owned(),
            }
        )
    }
}

impl<'a> ToString for SelectStmt<'a> {
    fn to_string(&self) -> String {
        self.str()
    }
}
