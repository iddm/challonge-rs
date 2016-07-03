macro_rules! format_date {
    ($date:expr) => {
        $date.format("%Y-%m-%d").to_string()
    }
}

macro_rules! t {
    ($key:expr) => {
        concat!("tournament[", $key, "]")
    }
}

macro_rules! p {
    ($key:expr) => {
        concat!("participant[", $key, "]")
    }
}

macro_rules! a {
    ($key:expr) => {
        concat!("match_attachment[", $key, "]")
    }
}

macro_rules! ps {
    ($key:expr) => {
        concat!("participant[][", $key, "]")
    }
}

macro_rules! m {
    ($key:expr) => {
        concat!("match[", $key, "]")
    }
}

macro_rules! builder {
    ($field:ident, $field_type:ty) => {
        /// A builder method for $field with `$field_type` type.
        pub fn $field<'a>(&'a mut self,
                          $field: $field_type) -> &'a mut Self {
            self.$field = $field;
            self
        }
    };
}

macro_rules! builder_s {
    ($field:ident) => {
        /// A builder method for $field with `String` type.
        pub fn $field<'a, S: Into<String>>(&'a mut self,
                                           $field: S) -> &'a mut Self {
            self.$field = $field.into();
            self
        }
    };
}

macro_rules! builder_o {
    ($field:ident, $field_type:ty) => {
        /// A builder method for $field with `Option` type.
        pub fn $field<'a>(&'a mut self,
                          $field: $field_type) -> &'a mut Self {
            self.$field = Some($field.into());
            self
        }
    };
}

macro_rules! builder_so {
    ($field:ident) => {
        /// A builder method for $field with `Option<String>` type.
        pub fn $field<'a, S: Into<String>>(&'a mut self,
                                           $field: S) -> &'a mut Self {
            self.$field = Some($field.into());
            self
        }
    };
}

