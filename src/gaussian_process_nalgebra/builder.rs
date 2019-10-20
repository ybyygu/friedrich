//! Fine Grained selection of the gaussian process parameters.

use crate::parameters::kernel::Kernel;
use crate::parameters::prior::Prior;
use super::GaussianProcess_nalgebra;
use nalgebra::{DMatrix, DVector};

/// Fine Grained selection of the gaussian process parameters.
pub struct GaussianProcessBuilder_nalgebra<KernelType: Kernel,
 PriorType: Prior>
{
   /// value to which the process will regress in the absence of informations
   prior: PriorType,
   /// kernel used to fit the process on the data
   kernel: KernelType,
   /// amplitude of the noise of the data
   noise: f64,
   /// type of fit to be applied
   should_fit_kernel: bool,
   should_fit_prior: bool,
   /// data use for training
   training_inputs: DMatrix<f64>,
   training_outputs: DVector<f64>
}

impl<KernelType: Kernel, PriorType: Prior>
   GaussianProcessBuilder_nalgebra<KernelType, PriorType>
{
   /// builds a new gaussian process with default parameters
   /// the defaults are :
   /// - constant prior set to 0
   /// - a gaussian kernel
   /// - a noise of 1e-7
   /// - does not fit parameters
   /// - does fit prior
   pub fn new(training_inputs: DMatrix<f64>, training_outputs: DVector<f64>) -> Self
   {
      let prior = PriorType::default(training_inputs.nrows());
      let kernel = KernelType::default();
      let noise = 1e-7f64;
      let should_fit_kernel = false;
      let should_fit_prior = false;
      GaussianProcessBuilder_nalgebra { prior,
                               kernel,
                               noise,
                               should_fit_kernel,
                               should_fit_prior,
                               training_inputs,
                               training_outputs }
   }

   //----------------------------------------------------------------------------------------------
   // SETTERS

   /// sets a new prior
   /// the prior is the value returned in the absence of information
   pub fn set_prior<PriorType2: Prior>(
      self,
      prior: PriorType2)
      -> GaussianProcessBuilder_nalgebra<KernelType, PriorType2>
   {
      GaussianProcessBuilder_nalgebra { prior,
                               kernel: self.kernel,
                               noise: self.noise,
                               should_fit_kernel: self.should_fit_kernel,
                               should_fit_prior: self.should_fit_prior,
                               training_inputs: self.training_inputs,
                               training_outputs: self.training_outputs }
   }

   /// sets the noise parameters which correspond to the magnitude of the noise in the data
   pub fn set_noise(self, noise: f64) -> Self
   {
      GaussianProcessBuilder_nalgebra { noise, ..self }
   }

   /// changes the kernel of the gaussian process
   pub fn set_kernel<KernelType2: Kernel>(
      self,
      kernel: KernelType2)
      -> GaussianProcessBuilder_nalgebra<KernelType2, PriorType>
   {
      GaussianProcessBuilder_nalgebra { prior: self.prior,
                               kernel,
                               noise: self.noise,
                               should_fit_kernel: self.should_fit_kernel,
                               should_fit_prior: self.should_fit_prior,
                               training_inputs: self.training_inputs,
                               training_outputs: self.training_outputs }
   }

   /// fits the parameters of the kernel on the training data
   pub fn fit_kernel(self) -> Self
   {
      GaussianProcessBuilder_nalgebra { should_fit_kernel: true, ..self }
   }

   /// fits the prior on the training data
   pub fn fit_prior(self) -> Self
   {
      GaussianProcessBuilder_nalgebra { should_fit_prior: true, ..self }
   }

   //----------------------------------------------------------------------------------------------
   // TRAIN

   /// trains the gaussian process
   pub fn train(self) -> GaussianProcess_nalgebra<KernelType, PriorType>
   {
      // builds a gp
      // TODO here we waste a training if we will fit anyway
      // TODO a new_fitted funtion might solve it
      // TODO or a raw construction that does not perform a fit
      let mut gp = GaussianProcess_nalgebra::<KernelType, PriorType>::new(self.prior,
                                                                                   self.kernel,
                                                                                   self.noise,
                                                                                   self.training_inputs,
                                                                                   self.training_outputs);
      // fit the model, if reqiested, on the training data
      gp.fit_parameters(self.should_fit_prior, self.should_fit_kernel);
      gp
   }
}
