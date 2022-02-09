#pragma once

using Foo = std::vector<int>;

enum class Something : uint8;

template <typename T>
struct BAR Foo : public Bar;

class FOO Bar;

template <typename T>
void* Main(const Foo& Arg);

/// Description of enum
///
/// More information and examples.
UENUM(BlueprintType, Meta = (Foo = Bar))
enum class Something : uint8
{
	A,
	B
};

/// Description of struct
///
/// More information and examples.
///
/// [`struct: Self::Foo`]()
/// [`struct: Self::A`]()
USTRUCT(BlueprintType, Meta = (Foo = Bar))
template <typename T>
struct BAR Foo : public Bar
{
protected:
	/// What is this method
	///
	/// What it does
	UFUNCTION()
	virtual void Foo(
		/// Argument
		int A,
		/// Argument with default value
		AActor* B = nullptr) const override;

private:
	/// What is this property
	///
	/// What impact does it have
	UPROPERTY()
	int A[] = {0};
};

/// Description of class
///
/// More information and examples.
///
/// [`class: Self::Bar`]()
UCLASS()
class FOO Bar
{
public:
	/// What is this method
	///
	/// What it does
	Bar();
};

/// What is this function
///
/// What does it do
///
/// [`function: Self`]()
///
/// See:
/// - [`enum: Something`]()
/// - [`struct: Foo`]()
/// - [`struct: Foo::Foo`]()
/// - [`struct: Foo::A`]()
/// - [`class: Bar`]()
/// - [`function: Main`]()
///
/// # Examples
/// ```snippet
/// hello_world
/// ```
/// ```snippet
/// hello_world2
/// ```
/// ```snippet
/// wait_what
/// ```
template <typename T>
void* Main(
	/// Some referenced data
	const Foo& Arg)
{
	//// [snippet: hello_world]
	if (true)
	{
		printf("Hello");
	}
	//// [/snippet]

	//// [snippet: hello_world2]
	printf("World");
	//// [/snippet]
}

//// [snippet: wait_what]
struct Wait
{
	int What = 0;
};
//// [/snippet]

/// Proxy documentation for injecting code with macros.
///
//// [proxy: injectable]
//// void Injected() const;
//// [/proxy]
#define INJECT            \
	void Injected() const \
	{                     \
	}

struct Who
{
	int What = 0;

	void SetWhat(int InWhat)
	{
		//// [ignore]
		this->What = InWhat;
		//// [/ignore]
	}

	//// [inject: injectable]
	INJECT
};
