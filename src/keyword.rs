#[derive(Debug, PartialEq)]
pub enum Keyword {
    Abort,            //abort
    Absolute,         //absolute
    Access,           //access
    Action,           //action
    Add,              //add
    Admin,            //admin
    After,            //after
    Aggregate,        //aggregate
    All,              //all
    Also,             //also
    Alter,            //alter
    Always,           //always
    Analyse,          //analyse
    Analyze,          //analyze
    And,              //and
    Any,              //any
    Array,            //array
    As,               //as
    Asc,              //asc
    Assertion,        //assertion
    Assignment,       //assignment
    Asymmetric,       //asymmetric
    At,               //at
    Attach,           //attach
    Attribute,        //attribute
    Authorization,    //authorization
    Backward,         //backward
    Before,           //before
    Begin,            //begin
    Between,          //between
    Bigint,           //bigint
    Binary,           //binary
    Bit,              //bit
    Boolean,          //boolean
    Both,             //both
    By,               //by
    Cache,            //cache
    Call,             //call
    Called,           //called
    Cascade,          //cascade
    Cascaded,         //cascaded
    Case,             //case
    Cast,             //cast
    Catalog,          //catalog
    Chain,            //chain
    Char,             //char
    Character,        //character
    Characteristics,  //characteristics
    Check,            //check
    Checkpoint,       //checkpoint
    Class,            //class
    Close,            //close
    Cluster,          //cluster
    Coalesce,         //coalesce
    Collate,          //collate
    Collation,        //collation
    Column,           //column
    Columns,          //columns
    Comment,          //comment
    Comments,         //comments
    Commit,           //commit
    Committed,        //committed
    Concurrently,     //concurrently
    Configuration,    //configuration
    Conflict,         //conflict
    Connection,       //connection
    Constraint,       //constraint
    Constraints,      //constraints
    Content,          //content
    Continue,         //continue
    Conversion,       //conversion
    Copy,             //copy
    Cost,             //cost
    Create,           //create
    Cross,            //cross
    Csv,              //csv
    Cube,             //cube
    Current,          //current
    CurrentCatalog,   //current_catalog
    CurrentDate,      //current_date
    CurrentRole,      //current_role
    CurrentSchema,    //current_schema
    CurrentTime,      //current_time
    CurrentTimestamp, //current_timestamp
    CurrentUser,      //current_user
    Cursor,           //cursor
    Cycle,            //cycle
    Data,             //data
    Database,         //database
    Day,              //day
    Deallocate,       //deallocate
    Dec,              //dec
    Decimal,          //decimal
    Declare,          //declare
    Default,          //default
    Defaults,         //defaults
    Deferrable,       //deferrable
    Deferred,         //deferred
    Definer,          //definer
    Delete,           //delete
    Delimiter,        //delimiter
    Delimiters,       //delimiters
    Depends,          //depends
    Desc,             //desc
    Detach,           //detach
    Dictionary,       //dictionary
    Disable,          //disable
    Discard,          //discard
    Distinct,         //distinct
    Do,               //do
    Document,         //document
    Domain,           //domain
    Double,           //double
    Drop,             //drop
    Each,             //each
    Else,             //else
    Enable,           //enable
    Encoding,         //encoding
    Encrypted,        //encrypted
    End,              //end
    Enum,             //enum
    Escape,           //escape
    Event,            //event
    Except,           //except
    Exclude,          //exclude
    Excluding,        //excluding
    Exclusive,        //exclusive
    Execute,          //execute
    Exists,           //exists
    Explain,          //explain
    Expression,       //expression
    Extension,        //extension
    External,         //external
    Extract,          //extract
    False,            //false
    Family,           //family
    Fetch,            //fetch
    Filter,           //filter
    First,            //first
    Float,            //float
    Following,        //following
    For,              //for
    Force,            //force
    Foreign,          //foreign
    Forward,          //forward
    Freeze,           //freeze
    From,             //from
    Full,             //full
    Function,         //function
    Functions,        //functions
    Generated,        //generated
    Global,           //global
    Grant,            //grant
    Granted,          //granted
    Greatest,         //greatest
    Group,            //group
    Grouping,         //grouping
    Groups,           //groups
    Handler,          //handler
    Having,           //having
    Header,           //header
    Hold,             //hold
    Hour,             //hour
    Identity,         //identity
    If,               //if
    Ilike,            //ilike
    Immediate,        //immediate
    Immutable,        //immutable
    Implicit,         //implicit
    Import,           //import
    In,               //in
    Include,          //include
    Including,        //including
    Increment,        //increment
    Index,            //index
    Indexes,          //indexes
    Inherit,          //inherit
    Inherits,         //inherits
    Initially,        //initially
    Inline,           //inline
    Inner,            //inner
    Inout,            //inout
    Input,            //input
    Insensitive,      //insensitive
    Insert,           //insert
    Instead,          //instead
    Int,              //int
    Integer,          //integer
    Intersect,        //intersect
    Interval,         //interval
    Into,             //into
    Invoker,          //invoker
    Is,               //is
    Isnull,           //isnull
    Isolation,        //isolation
    Join,             //join
    Key,              //key
    Label,            //label
    Language,         //language
    Large,            //large
    Last,             //last
    Lateral,          //lateral
    Leading,          //leading
    Leakproof,        //leakproof
    Least,            //least
    Left,             //left
    Level,            //level
    Like,             //like
    Limit,            //limit
    Listen,           //listen
    Load,             //load
    Local,            //local
    Localtime,        //localtime
    Localtimestamp,   //localtimestamp
    Location,         //location
    Lock,             //lock
    Locked,           //locked
    Logged,           //logged
    Mapping,          //mapping
    Match,            //match
    Materialized,     //materialized
    Maxvalue,         //maxvalue
    Method,           //method
    Minute,           //minute
    Minvalue,         //minvalue
    Mode,             //mode
    Month,            //month
    Move,             //move
    Name,             //name
    Names,            //names
    National,         //national
    Natural,          //natural
    Nchar,            //nchar
    New,              //new
    Next,             //next
    Nfc,              //nfc
    Nfd,              //nfd
    Nfkc,             //nfkc
    Nfkd,             //nfkd
    No,               //no
    None,             //none
    Normalize,        //normalize
    Normalized,       //normalized
    Not,              //not
    Nothing,          //nothing
    Notify,           //notify
    Notnull,          //notnull
    Nowait,           //nowait
    Null,             //null
    Nullif,           //nullif
    Nulls,            //nulls
    Numeric,          //numeric
    Object,           //object
    Of,               //of
    Off,              //off
    Offset,           //offset
    Oids,             //oids
    Old,              //old
    On,               //on
    Only,             //only
    Operator,         //operator
    Option,           //option
    Options,          //options
    Or,               //or
    Order,            //order
    Ordinality,       //ordinality
    Others,           //others
    Out,              //out
    Outer,            //outer
    Over,             //over
    Overlaps,         //overlaps
    Overlay,          //overlay
    Overriding,       //overriding
    Owned,            //owned
    Owner,            //owner
    Parallel,         //parallel
    Parser,           //parser
    Partial,          //partial
    Partition,        //partition
    Passing,          //passing
    Password,         //password
    Placing,          //placing
    Plans,            //plans
    Policy,           //policy
    Position,         //position
    Preceding,        //preceding
    Precision,        //precision
    Prepare,          //prepare
    Prepared,         //prepared
    Preserve,         //preserve
    Primary,          //primary
    Prior,            //prior
    Privileges,       //privileges
    Procedural,       //procedural
    Procedure,        //procedure
    Procedures,       //procedures
    Program,          //program
    Publication,      //publication
    Quote,            //quote
    Range,            //range
    Read,             //read
    Real,             //real
    Reassign,         //reassign
    Recheck,          //recheck
    Recursive,        //recursive
    Ref,              //ref
    References,       //references
    Referencing,      //referencing
    Refresh,          //refresh
    Reindex,          //reindex
    Relative,         //relative
    Release,          //release
    Rename,           //rename
    Repeatable,       //repeatable
    Replace,          //replace
    Replica,          //replica
    Reset,            //reset
    Restart,          //restart
    Restrict,         //restrict
    Returning,        //returning
    Returns,          //returns
    Revoke,           //revoke
    Right,            //right
    Role,             //role
    Rollback,         //rollback
    Rollup,           //rollup
    Routine,          //routine
    Routines,         //routines
    Row,              //row
    Rows,             //rows
    Rule,             //rule
    Savepoint,        //savepoint
    Schema,           //schema
    Schemas,          //schemas
    Scroll,           //scroll
    Search,           //search
    Second,           //second
    Security,         //security
    Select,           //select
    Sequence,         //sequence
    Sequences,        //sequences
    Serializable,     //serializable
    Server,           //server
    Session,          //session
    SessionUser,      //session_user
    Set,              //set
    SetOf,            //setof
    Sets,             //sets
    Share,            //share
    Show,             //show
    Similar,          //similar
    Simple,           //simple
    Skip,             //skip
    Smallint,         //smallint
    Snapshot,         //snapshot
    Some,             //some
    Sql,              //sql
    Stable,           //stable
    Standalone,       //standalone
    Start,            //start
    Statement,        //statement
    Statistics,       //statistics
    Stdin,            //stdin
    Stdout,           //stdout
    Storage,          //storage
    Stored,           //stored
    Strict,           //strict
    Strip,            //strip
    Subscription,     //subscription
    Substring,        //substring
    Support,          //support
    Symmetric,        //symmetric
    Sysid,            //sysid
    System,           //system
    Table,            //table
    Tables,           //tables
    TableSample,      //tablesample
    Tablespace,       //tablespace
    Temp,             //temp
    Template,         //template
    Temporary,        //temporary
    Text,             //text
    Then,             //then
    Ties,             //ties
    Time,             //time
    Timestamp,        //timestamp
    To,               //to
    Trailing,         //trailing
    Transaction,      //transaction
    Transform,        //transform
    Treat,            //treat
    Trigger,          //trigger
    Trim,             //trim
    True,             //true
    Truncate,         //truncate
    Trusted,          //trusted
    Type,             //type
    Types,            //types
    Uescape,          //uescape
    Unbounded,        //unbounded
    Uncommitted,      //uncommitted
    Unencrypted,      //unencrypted
    Union,            //union
    Unique,           //unique
    Unknown,          //unknown
    Unlisten,         //unlisten
    Unlogged,         //unlogged
    Until,            //until
    Update,           //update
    User,             //user
    Using,            //using
    Vacuum,           //vacuum
    Valid,            //valid
    Validate,         //validate
    Validator,        //validator
    Value,            //value
    Values,           //values
    Varchar,          //varchar
    Variadic,         //variadic
    Varying,          //varying
    Verbose,          //verbose
    Version,          //version
    View,             //view
    Views,            //views
    Volatile,         //volatile
    When,             //when
    Where,            //where
    Whitespace,       //whitespace
    Window,           //window
    With,             //with
    Within,           //within
    Without,          //without
    Work,             //work
    Wrapper,          //wrapper
    Write,            //write
    Xml,              //xml
    XmlAttributes,    //xmlattributes
    XmlConcat,        //xmlconcat
    XmlElement,       //xmlelement
    XmlExists,        //xmlexists
    XmlForest,        //xmlforest
    XmlNamespaces,    //xmlnamespaces
    XmlParse,         //xmlparse
    XmlPi,            //xmlpi
    XmlRoot,          //xmlroot
    XmlSerialize,     //xmlserialize
    XmlTable,         //xmltable
    Year,             //year
    Yes,              //yes
    Zone,             //zone
}
