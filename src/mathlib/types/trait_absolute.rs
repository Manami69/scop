use crate::mathlib::classes::complex::Complexf32;

/// Absolute value of f32 or complex f32
pub trait Absolute {
    fn abs(self) -> f32;
}

impl Absolute for f32 {
    fn abs(self) -> f32 {
        if self < 0.0 {
            -self
        } else {
            self
        }
    }
}

impl Absolute for Complexf32 {
    /// En mathématiques, le module d'un nombre complexe est le nombre réel positif qui mesure sa « taille »
    /// et généralise la valeur absolue d'un nombre réel.
    /// Cette notion est notamment utile pour définir une distance sur le plan complexe.
    ///
    /// Le module d'un nombre complexe z est noté |z|.
    /// Si le complexe z s'exprime sous sa forme algébrique, a + ib, où i est l'unité imaginaire,
    /// a est la partie réelle de z et b sa partie imaginaire, ce module est la racine carrée de la somme des carrés de a et b.
    ///
    /// https://fr.wikipedia.org/wiki/Module_d%27un_nombre_complexe
    fn abs(self) -> f32 {
        (self.real * self.real + self.imaginary * self.imaginary).sqrt()
    }
}
